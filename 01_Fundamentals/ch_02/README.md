# Programming a Guessing Game

This chapter is a hands-on practice on Rust programmming, introducing some basic concepts of the language.

## Processing a Guess

### The `std::io` module
By default, Rust has a **set of items defined in the `standard library`** that it brings into the scope of every program. This set is called the *prelude*. Its documentation and features can be accessed [here](https://doc.rust-lang.org/std/prelude/index.html).

### Storing values with Variables

In Rust, variables are used to store data. You can create a variable using the `let` **statement**. By default, variables are immutable, meaning their values cannot be changed once assigned. To make a variable mutable, you can use the `mut` keyword **before the variable name**. For example:

```rust
let mut guess = String::new();
```

The `::` syntax in the `::new` line indicates that `new` is an associated function of the `String` type. Associated functions are defined within the context of a type and are called using the `::` syntax. Briefly:

* `String::new()` creates a new instance of the `String` type.
* `String` is a type that represents a growable, heap-allocated string in Rust. It is part of the standard library and provides various methods for manipulating strings.

### Receiving User Input

To receive input from the user, Rust provides the `std::io` module, which contains functions and types for handling input and output. One common way to get user input is by using the `stdin` function, which returns a handle to the standard input. You can then call the `read_line` method on this handle to read a line of input from the user. For example:

```rust
use std::io;

fn main() {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
}
```

* `io::stdin()` returns a handle to the standard input. Handles are used to interact with input and output streams in Rust.
* `read_line(&mut guess)` reads a line of input from the user and appends it to the `guess` variable. The `&mut guess` syntax indicates that we are passing a mutable reference to the `guess` variable, allowing the `read_line` method to modify its value. Breaking down into smaller parts:
    * `&` indicates that we are passing a reference to the variable, rather than the variable itself. This allows us to avoid moving ownership of the variable and enables us to use it later in the program.
    * `mut` indicates that the reference is mutable, allowing the `read_line` method to modify the value of the variable.

### Handling Errors

In Rust, error handling is an important aspect of writing robust programs. The `expect` method is used to handle potential errors that may occur during the execution of a program. In the example above, `expect("Failed to read line")` is called on the result of `read_line`. If an error occurs, the program will terminate and display the provided error message. This is a simple way to handle errors during development, but in production code, more sophisticated error handling techniques are often used.

As the `read_line` returns a `Result` enum type, it can either be `Ok` or `Err`. The `expect` method is a convenient way to handle the `Result` type, as it will automatically panic if the result is an error. This allows developers to quickly identify and fix issues during development.

### Printing values with `println!` placeholders

The `{}` syntax is used to create placeholders in a string, which can be replaced with values at runtime. The `println!` macro is used to print formatted output to the console. For example:

```rust
println!("You guessed: {guess}");
```

If the variable or placeholder's value itself changes over time, the `println!` macro will always print the current value of the variable at the time of execution. This allows for dynamic output based on the program's state.

```rust
println!("You guessed: {}", guess * 2);
```

## Generating a Secret Number

Next, we will generate a random number that the user will try to guess. To do this, we will use the `rand` crate, which provides functionality for generating random numbers.

When using external crates, you need to add them as dependencies in your `Cargo.toml` file. In this case, we will add the `rand` crate with the following line:

```toml
[dependencies]
rand = "0.8.5"
```

### Updating a Crate to Get a New Version
Then, when you want to update a crate to get a new version, you can use the `cargo update` command. This command will update the dependencies in your `Cargo.lock` file to the latest compatible versions based on the version requirements specified in your `Cargo.toml` file.

Let's start using the `rand` crate to generate a random number. First, we need to import the necessary items from the `rand` crate. We will use the `Rng` trait, which provides methods for generating random numbers.

```rust
use rand::Rng;

let secret_number = rand::thread_rng().gen_range(1..=100);
```

## Comparing values

In order to compare values, we use `match` expressions. The way we will use here is from `std::cmp` module, which provides the `Ordering` enum. The `Ordering` enum has three variants: `Less`, `Equal`, and `Greater`. We can use the `cmp` method to compare two values and return an `Ordering` value.

```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) { // Guess is the variable that stores the user's input, and secret_number is the randomly generated number.
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

A `match` expression is made up of **arms**. An arm consists of a `pattern` to match against, and the code that should be run if the value is given to `match` fits that arm's pattern. Comparing to JavaScript ecossystem, the `match` expression is similar to a `switch` statement, but it is more powerful and flexible. It allows for pattern matching, which can be used to destructure complex data types and match against specific values or conditions.

However, if we run the code above, we will get an error because the `guess` variable is a `String`, and we need to convert it to a number before we can compare it with the `secret_number`. We can use the `parse` method to convert the `String` to a number. The `parse` method returns a `Result` type, which we can handle using a `match` expression.

```rust
let guess: u32 = match guess.trim().parse().expect("Please type a number!");
```

`Rust` allows us to shadow the previous value of `guess` with a new one. Shadowing lets us reuse the `guess` variable name, rather than creating a new variable. This can be useful when we want to transform a value and keep the same variable name for clarity.


## Allowing the User to Guess Again

The `loop` keyword is used to create an infinite loop, which will allow the user to keep guessing until they get the correct answer. We can use the `break` statement to exit the loop when the user guesses the correct number.

```rust
loop {
    // Get user input and compare with secret_number
    let guess: u32 = match guess.trim().parse().expect("Please type a number!");
    if guess == secret_number {
        println!("You win!");
        break; // Exit the loop
    }
}
```


## Summary - Table of Concepts

| Concept | Description |
|---------|-------------|
| `std::io` | A module in the Rust standard library that provides functionality for input and output operations. |
| `let` statement | Used to declare variables in Rust. By default, variables are immutable. |
| `match` expression | A control flow construct that allows you to compare a value against a series of patterns and execute code based on which pattern matches. |
| `loop` keyword | Used to create an infinite loop, which can be exited using the `break` statement. |
| Shadowing | Allows you to reuse a variable name by declaring a new variable with the same name, effectively "shadowing" the previous variable. |
| `rand` crate | An external crate that provides functionality for generating random numbers. It can be added as a dependency in the `Cargo.toml` file. |
| `Ordering` enum | An enum in the `std::cmp` module that represents the result of a comparison between two values. It has three variants: `Less`, `Equal`, and `Greater`. |