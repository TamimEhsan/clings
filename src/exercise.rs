use anyhow::Result;
use crossterm::{
    QueueableCommand,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
};
use std::io::{self, StdoutLock, Write};

use crate::{
    cmd::CmdRunner,
    term::{self, CountedWrite, file_path, terminal_file_link, write_ansi},
};

/// The initial capacity of the output buffer.
pub const OUTPUT_CAPACITY: usize = 1 << 14;

pub fn solution_link_line(
    stdout: &mut StdoutLock,
    solution_path: &str,
    emit_file_links: bool,
) -> io::Result<()> {
    stdout.queue(SetAttribute(Attribute::Bold))?;
    stdout.write_all(b"Solution")?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b" for comparison: ")?;
    file_path(stdout, Color::Cyan, |writer| {
        if emit_file_links && let Some(canonical_path) = term::canonicalize(solution_path) {
            terminal_file_link(writer, solution_path, &canonical_path)
        } else {
            writer.stdout().write_all(solution_path.as_bytes())
        }
    })?;
    stdout.write_all(b"\n")
}

// Run an exercise binary and append its output to the `output` buffer.
// Compilation must be done before calling this method.
fn run_bin(
    bin_name: &str,
    mut output: Option<&mut Vec<u8>>,
    cmd_runner: &CmdRunner,
) -> Result<bool> {
    if let Some(output) = output.as_deref_mut() {
        write_ansi(output, SetAttribute(Attribute::Underlined));
        output.extend_from_slice(b"Output");
        write_ansi(output, ResetColor);
        output.push(b'\n');
    }

    let success = cmd_runner.run_bin(bin_name, output.as_deref_mut())?;

    if let Some(output) = output
        && !success
    {
        // This output is important to show the user that something went wrong.
        // Otherwise, calling something like `exit(1)` in an exercise without further output
        // leaves the user confused about why the exercise isn't done yet.
        write_ansi(output, SetAttribute(Attribute::Bold));
        write_ansi(output, SetForegroundColor(Color::Red));
        output.extend_from_slice(b"The exercise didn't run successfully (nonzero exit code)");
        write_ansi(output, ResetColor);
        output.push(b'\n');
    }

    Ok(success)
}

/// See `info_file::ExerciseInfo`
pub struct Exercise {
    pub dir: Option<&'static str>,
    pub name: &'static str,
    /// Path of the exercise file starting with the `exercises/` directory.
    pub path: &'static str,
    pub canonical_path: Option<String>,
    pub test: bool,
    pub hint: &'static str,
    pub done: bool,
}

impl Exercise {
    pub fn terminal_file_link<'a>(
        &self,
        writer: &mut impl CountedWrite<'a>,
        emit_file_links: bool,
    ) -> io::Result<()> {
        file_path(writer, Color::Blue, |writer| {
            if emit_file_links && let Some(canonical_path) = self.canonical_path.as_deref() {
                terminal_file_link(writer, self.path, canonical_path)
            } else {
                writer.write_str(self.path)
            }
        })
    }
}

pub trait RunnableExercise {
    fn name(&self) -> &str;
    fn dir(&self) -> Option<&str>;
    fn test(&self) -> bool;
    fn path(&self) -> String;

    // Compile, check and run the exercise or its solution (depending on `bin_name´).
    // The output is written to the `output` buffer after clearing it.
    fn run<const FORCE_STRICT_CLIPPY: bool>(
        &self,
        bin_name: &str,
        mut output: Option<&mut Vec<u8>>,
        cmd_runner: &CmdRunner,
    ) -> Result<bool> {
        if let Some(output) = output.as_deref_mut() {
            output.clear();
        }
        let exercise_path = self.path();
        
        // Compile C file using gcc
        let mut compile_cmd = cmd_runner.gcc(bin_name, output.as_deref_mut());
        
        compile_cmd.args([exercise_path.to_string().as_str()]);
        
        // Execute compilation
        let compile_success = compile_cmd.run("gcc compile")?;
        
        if !compile_success {
            return Ok(false);
        }
        // Run the compiled binary
        let run_success = run_bin(bin_name, output, cmd_runner)?;
        Ok(run_success)
    }

    /// Compile, check and run the exercise.
    /// The output is written to the `output` buffer after clearing it.
    #[inline]
    fn run_exercise(&self, output: Option<&mut Vec<u8>>, cmd_runner: &CmdRunner) -> Result<bool> {
        self.run::<false>(self.name(), output, cmd_runner)
    }

    /// Compile, check and run the exercise's solution.
    /// The output is written to the `output` buffer after clearing it.
    fn run_solution(&self, output: Option<&mut Vec<u8>>, cmd_runner: &CmdRunner) -> Result<bool> {
        let name = self.name();
        let mut bin_name = String::with_capacity(name.len() + 4);
        bin_name.push_str(name);
        bin_name.push_str("_sol");

        self.run::<true>(&bin_name, output, cmd_runner)
    }

    fn sol_path(&self) -> String {
        let name = self.name();

        let mut path = if let Some(dir) = self.dir() {
            // 14 = 10 + 1 + 3
            // solutions/ + / + .c
            let mut path = String::with_capacity(13 + dir.len() + name.len());
            path.push_str("solutions/");
            path.push_str(dir);
            path.push('/');
            path
        } else {
            // 13 = 10 + 3
            // solutions/ + .c
            let mut path = String::with_capacity(12 + name.len());
            path.push_str("solutions/");
            path
        };

        path.push_str(name);
        path.push_str(".c"); // Changed from .rs to .c

        path
    }
}

impl RunnableExercise for Exercise {
    #[inline]
    fn name(&self) -> &str {
        self.name
    }

    #[inline]
    fn dir(&self) -> Option<&str> {
        self.dir
    }

    #[inline]
    fn test(&self) -> bool {
        self.test
    }

    #[inline]
    fn path(&self) -> String {
        self.path.to_string()
    }
}
