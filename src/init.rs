use anyhow::{Context, Result, bail};
use crossterm::{
    QueueableCommand,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
};
use serde::Deserialize;
use std::{
    env::set_current_dir,
    fs::{self, create_dir},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{
    cargo_toml::updated_cargo_toml, embedded::EMBEDDED_FILES, exercise::RunnableExercise,
    info_file::InfoFile, term::press_enter_prompt,
};

#[derive(Deserialize)]
struct CargoLocateProject {
    root: PathBuf,
}

pub fn init() -> Result<()> {
    let clings_dir = Path::new("clings");
    if clings_dir.exists() {
        bail!(CLINGS_DIR_ALREADY_EXISTS_ERR);
    }

    let locate_project_output = Command::new("cargo")
        .arg("locate-project")
        .arg("-q")
        .arg("--workspace")
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .context(
            "Failed to run the command `cargo locate-project …`\n\
             Did you already install Rust?\n\
             Try running `cargo --version` to diagnose the problem.",
        )?;

    if !Command::new("cargo")
        .arg("clippy")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Failed to run the command `cargo clippy --version`")?
        .success()
    {
        bail!(
            "Clippy, the official Rust linter, is missing.\n\
             Please install it first before initializing Clings."
        )
    }

    let mut stdout = io::stdout().lock();
    let mut init_git = true;

    if locate_project_output.status.success() {
        if Path::new("exercises").exists() && Path::new("solutions").exists() {
            bail!(IN_INITIALIZED_DIR_ERR);
        }

        let workspace_manifest =
            serde_json::de::from_slice::<CargoLocateProject>(&locate_project_output.stdout)
                .context(
                    "Failed to read the field `root` from the output of `cargo locate-project …`",
                )?
                .root;

        let workspace_manifest_content = fs::read_to_string(&workspace_manifest)
            .with_context(|| format!("Failed to read the file {}", workspace_manifest.display()))?;
        if !workspace_manifest_content.contains("[workspace]")
            && !workspace_manifest_content.contains("workspace.")
        {
            bail!(
                "The current directory is already part of a Cargo project.\n\
                 Please initialize Clings in a different directory"
            );
        }

        stdout.write_all(b"This command will create the directory `clings/` as a member of this Cargo workspace.\n\
                           Press ENTER to continue ")?;
        press_enter_prompt(&mut stdout)?;

        // Make sure "clings" is added to `workspace.members` by making
        // Cargo initialize a new project.
        let status = Command::new("cargo")
            .arg("new")
            .arg("-q")
            .arg("--vcs")
            .arg("none")
            .arg("clings")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .status()?;
        if !status.success() {
            bail!(
                "Failed to initialize a new Cargo workspace member.\n\
                 Please initialize Clings in a different directory"
            );
        }

        stdout.write_all(b"The directory `clings` has been added to `workspace.members` in the `Cargo.toml` file of this Cargo workspace.\n")?;
        fs::remove_dir_all("clings")
            .context("Failed to remove the temporary directory `clings/`")?;
        init_git = false;
    } else {
        stdout.write_all(b"This command will create the directory `clings/` which will contain the exercises.\n\
                           Press ENTER to continue ")?;
        press_enter_prompt(&mut stdout)?;
    }

    create_dir(clings_dir).context("Failed to create the `clings/` directory")?;
    set_current_dir(clings_dir)
        .context("Failed to change the current directory to `clings/`")?;

    let info_file = InfoFile::parse()?;
    EMBEDDED_FILES
        .init_exercises_dir(&info_file.exercises)
        .context("Failed to initialize the `clings/exercises` directory")?;

    create_dir("solutions").context("Failed to create the `solutions/` directory")?;
    fs::write(
        "solutions/README.md",
        include_bytes!("../solutions/README.md"),
    )
    .context("Failed to create the file clings/solutions/README.md")?;
    for dir in EMBEDDED_FILES.exercise_dirs {
        let mut dir_path = String::with_capacity(10 + dir.name.len());
        dir_path.push_str("solutions/");
        dir_path.push_str(dir.name);
        create_dir(&dir_path)
            .with_context(|| format!("Failed to create the directory {dir_path}"))?;
    }
    for exercise_info in &info_file.exercises {
        let solution_path = exercise_info.sol_path();
        fs::write(&solution_path, INIT_SOLUTION_FILE)
            .with_context(|| format!("Failed to create the file {solution_path}"))?;
    }

    let current_cargo_toml = include_str!("../dev-Cargo.toml");
    // Skip the first line (comment).
    let newline_ind = current_cargo_toml
        .as_bytes()
        .iter()
        .position(|c| *c == b'\n')
        .context("The embedded `Cargo.toml` is empty or contains only one line")?;
    let current_cargo_toml = current_cargo_toml
        .get(newline_ind + 1..)
        .context("The embedded `Cargo.toml` contains only one line")?;
    let updated_cargo_toml = updated_cargo_toml(&info_file.exercises, current_cargo_toml, b"")
        .context("Failed to generate `Cargo.toml`")?;
    fs::write("Cargo.toml", updated_cargo_toml)
        .context("Failed to create the file `clings/Cargo.toml`")?;

    fs::write("rust-analyzer.toml", RUST_ANALYZER_TOML)
        .context("Failed to create the file `clings/rust-analyzer.toml`")?;

    fs::write(".gitignore", GITIGNORE)
        .context("Failed to create the file `clings/.gitignore`")?;

    create_dir(".vscode").context("Failed to create the directory `clings/.vscode`")?;
    fs::write(".vscode/extensions.json", VS_CODE_EXTENSIONS_JSON)
        .context("Failed to create the file `clings/.vscode/extensions.json`")?;

    if init_git {
        // Ignore any Git error because Git initialization is not required.
        let _ = Command::new("git")
            .arg("init")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    stdout.queue(SetForegroundColor(Color::Green))?;
    stdout.write_all("Initialization done ✓".as_bytes())?;
    stdout.queue(ResetColor)?;
    stdout.write_all(b"\n\n")?;

    stdout.queue(SetAttribute(Attribute::Bold))?;
    stdout.write_all(POST_INIT_MSG)?;
    stdout.queue(ResetColor)?;

    Ok(())
}

const INIT_SOLUTION_FILE: &[u8] = b"void main() {
    // DON'T EDIT THIS SOLUTION FILE!
    // It will be automatically filled after you finish the exercise.
}
";

pub const RUST_ANALYZER_TOML: &[u8] = br#"check.command = "clippy"
check.extraArgs = ["--profile", "test"]
cargo.targetDir = true
"#;

const GITIGNORE: &[u8] = b"Cargo.lock
target/
.vscode/
";

pub const VS_CODE_EXTENSIONS_JSON: &[u8] = br#"{"recommendations":["rust-lang.rust-analyzer"]}"#;

const IN_INITIALIZED_DIR_ERR: &str = "It looks like Clings is already initialized in this directory.

If you already initialized Clings, run the command `clings` for instructions on getting started with the exercises.
Otherwise, please run `clings init` again in a different directory.";

const CLINGS_DIR_ALREADY_EXISTS_ERR: &str =
    "A directory with the name `clings` already exists in the current directory.
You probably already initialized Clings.
Run `cd clings`
Then run `clings` again";

const POST_INIT_MSG: &[u8] = b"Run `cd clings` to go into the generated directory.
Then run `clings` to get started.
";
