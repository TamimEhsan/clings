# Variables

C variables are strongly typed, which means that every variable must be declared with a specific data type. The most common data types in C are:
- `int`: for integers (whole numbers)
- `float`: for floating-point numbers (numbers with decimal points)
- `char`: for single characters
- `double`: for double-precision floating-point numbers
- `bool`: for boolean values (true or false)

## Declaring Variables
To declare a variable in C, you need to specify its data type followed by the variable name. For example:
```c
int age;
float height;
```

You can also initialize a variable at the time of declaration:
```c
int age = 25;
float height = 5.9;
```

## Auto-casting and Down-casting
C performs implicit type conversion (auto-casting) when you assign a value of one data type to a variable of another data type. For example:
```c
int a = 10;
float b = a; // 'a' is auto-cast to float
```
However, C does not support down-casting (explicit type conversion) in the same way as some other languages. You can use type casting to convert a variable from one type to another, but you need to do it explicitly:
```c
float x = 5.7;
int y = (int)x; // Explicitly down-casting float to int
```
