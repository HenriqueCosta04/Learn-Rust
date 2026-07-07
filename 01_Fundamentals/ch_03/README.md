# Common Programming Concepts

This chapter covers some of the most common programming concepts that are used in many programming languages. These concepts include variables, data types, control structures, functions, and more - applied to Rust!

## Variables and Mutability

In Rust, by default, variables are `immutable`, meaning that once a value is assigned to a variable, it cannot be changed. However, you can make a variable mutable by using the `mut` keyword.

```rust
fn main() {
    let x = 5; // immutable variable
    println!("The value of x is: {}", x);
    
    let mut y = 10; // mutable variable
    println!("The value of y is: {}", y);
    
    y += 5; // modifying the mutable variable
    println!("The new value of y is: {}", y);
}
```

The piece of code above will output a `compile-time error` if you try to modify an immutable variable. It’s important that we get compile-time errors when we attempt to change a value that’s designated as immutable, because this very situation can lead to bugs. The Rust compiler guarantees that when you state that a value won’t change, it really won’t change, so you don’t have to keep track of it yourself. Your code is thus easier to reason through.

But `mutability` can be very useful and can make code more convenient to write. Although variables are immutable by default, you can make them mutable by using the `mut` keyword. This allows you to change the value of a variable after it has been initialized.

### Declaring `constants`

Like immutable variables, `constants` are values that are **bound to a name and are not allowed to change**. However, there are a few differences between constants and immutable variables:

- Constants are always immutable.
- You declare constants using the `const` keyword instead of the `let` keyword.
- The constant type must be annotated (e.g., `const MAX_POINTS: u32 = 100_000;`).
- Constants can be declared in any scope, including the global scope, and are valid for the entire duration of a program.
- Constants may be set only to a constant expression, not the result of a function call or any other value that could only be computed at runtime.

```rust
const MAX_POINTS: u32 = 100_000;
```

Rust's naming convention for constants is to use **all uppercase with underscores between words**. This makes it easy to distinguish constants from variables.
Constants are valid for the entire time a program runs, within the scope in which they were declared. 

### Shadowing

`Shadowing` is a feature in Rust that allows you to declare a new variable with the same name as a previous variable. The new variable shadows the previous one, meaning that the previous variable is no longer accessible. This can be useful for transforming a value while keeping the same name.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have completed.

The other difference between `mut` and `shadowing` is that with `mut`, we can change the value of a variable without changing its type, while with `shadowing`, we can change the type of a variable as well.

```rust
fn main() {
    let spaces = "   "; // spaces is a string slice
    let spaces = spaces.len(); // spaces is now an integer
}
```

## Data Types

`Data Types` tell Rust what kind of data is being specified so that it knows how to work with that data. We'll look at two data type subsets: `scalar` and `compound` types. Rust is a `statically typed` language, which means that it must know the types of all variables at compile time. 

### Scalar Types
A `scalar` type represents a **single value**. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

#### Integer Types
An `integer` is a number without a fractional component. Rust has several integer types, which can be signed or unsigned and have different sizes. The most common integer types are:

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| 128-bit | i128   | u128     |
| Architecture-dependent | isize | usize |

Each variant can be either signed or unsigned. **Signed** integers can represent **both positive and negative numbers**, while **unsigned** integers can **only represent positive numbers and zero**. The size of the integer type determines the range of values it can hold. It's like **writing numbers on a paper**, when the sign matters a number is written with a `+` or `-` sign, and when the sign doesn't matter, we can write it without a sign.

Each `sign variant` can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive, where **n** is the number of bits that the type uses. For example, an `i8` can store numbers from −(2^8 − 1) to 2^8 − 1 − 1, which is −128 to 127. An `u8` can store numbers from 0 to 2^8 − 1, which is 0 to 255.

Additionally, the `isize` and `usize` types depend on the architecture of the computer your program is running on. On a 32-bit architecture, these types will be 32 bits wide, while on a 64-bit architecture, they will be 64 bits wide. These types are primarily used for indexing collections.

> **Question:** So how do you know which type of integer to use?
> If you’re unsure, Rust’s defaults are generally good places to start: Integer types default to i32. The primary situation in which you’d use isize or usize is when indexing some sort of collection.

#### Floating-Point Types

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

```rust
    fn main() {
        let x = 2.0; // f64 by default
        let y: f32 = 3.0; // f32
    }
```

#### Numeric Operations
Rust supports the basic mathematical operations you’d expect for all the number types: addition, subtraction, multiplication, division, and remainder. Integer division truncates toward zero to the nearest integer.

#### Boolean Type
Booleans are one byte in size. The Boolean type in Rust is specified using `bool`, and it can have one of two possible values: `true` and `false`. 

```rust
    fn main() {
        let t = true;
        let f: bool = false; // with explicit type annotation
    }
```

#### Character Type
Rust’s char type is the language’s most primitive alphabetic type. Here are some examples of declaring char values:

```rust
    fn main() {
        let c = 'z';
        let z: char = 'Z'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }
```

Note that we specify char literals with single quotation marks, as opposed to string literals, which use double quotation marks. Rust’s char type is 4 bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII.

### Compound Types

**Compound types** can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

#### Tuple Type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. **Tuples have a fixed length**: Once declared, they cannot grow or shrink in size.

```rust
  fn main() {
      let my_tuple: (i32, &str, f64) = (500, "Hello", 6.4);
      let (x, y, z) = my_tuple; // destructuring the tuple

        println!("The value of y is: {y}");
  }
```

The tuple without any value has a special name: `unit`. This value and its corresponding type are both written as `()`. The unit type is used when there is no other meaningful value that could be returned. Functions that don’t return a value return the unit type.


#### Array Type
Unlike tuples, every element of an array must have the same type. Arrays in Rust have a fixed length, meaning that once declared, they cannot grow or shrink in size. The length of an array is part of its type signature.

```rust
  fn main() {
      let my_array: [i32; 5] = [1, 2, 3, 4, 5];
      let first_element = my_array[0]; // accessing the first element
      println!("The first element is: {first_element}");
  }
```

If we try to access an element of an array using an index that is outside the bounds of the array, Rust will panic and terminate the program. This is a safety feature that prevents undefined behavior and helps catch bugs early in the development process.


### Data Types Summary

Below is a summary of the data types we have covered in this chapter:

| Type | Description | Example |
|------|-------------|---------|
| Scalar | Represents a single value | `i32`, `f64`, `bool`, `char` |
| Compound | Groups multiple values into one type | `tuple`, `array` |
| Integer | Represents whole numbers | `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`, `isize`, `usize` |
| Floating-Point | Represents numbers with decimal points | `f32`, `f64` |
| Boolean | Represents true or false values | `bool` |
| Character | Represents a single Unicode character | `char` |
| Tuple | Groups multiple values of different types | `(i32, &str, f64)`, `(bool, char)` |
| Array | Groups multiple values of the same type | `[i32; 5]`, `[bool; 3] or [bool, bool, bool]`  |
| Unit | Represents an empty value | `()` |


## Functions 

Rust code uses `snake case` as the conventional style for function and variable names. Its base syntax is defined as below: 

```rust
fn function_name(parameter_1, parameter_2, parameter_n) {
    // Function scope.
}
```

### Parameters

#### Parameters vs. Arguments

We can define functions to have `parameters`, which are variables that act as placeholders for the values that will be passed to the function when it is called. When we call a function, we provide `arguments`, which are the actual values that are passed to the function's parameters.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

In function signatures, we **must** declare the type of each parameter, as Rust Design Principles require that the compiler knows the types of all variables at compile time. This is part of Rust's strong type system, which helps catch errors early in the development process.

#### Statements and Expressions

Function bodies are made up of a series of `statements` and `expressions`. A statement is an instruction that performs some action but does not return a value, while an expression evaluates to a value. In Rust, most things are expressions, including function calls and blocks.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // This is an expression that evaluates to 4
    };

    println!("The value of y is: {y}");
}
```

Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression.

##### Functions with Return values

Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. 

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```

There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i32. 

### Functions Summary

| Concept | Description | Example |
|---------|-------------|---------|
| Function | A named block of code that can be called with arguments and may return a value | `fn function_name(parameter_1, parameter_2) -> return_type { /* function body */ }` |
| Parameter | A variable that acts as a placeholder for a value passed to a function | `fn another_function(x: i32) { /* function body */ }` |
| Argument | The actual value passed to a function's parameter when the function is called | `another_function(5);` |
| Statement | An instruction that performs an action but does not return a value | `let x = 5;` |
| Expression | A piece of code that evaluates to a value | `let y = { let x = 3; x + 1 };` |
| Return Value | The value returned by a function to the code that calls it | `fn five() -> i32 { 5 }` |