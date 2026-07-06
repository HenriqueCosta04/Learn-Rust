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