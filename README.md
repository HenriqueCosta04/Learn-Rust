# Rust Roadmap - From the basics to advanced concepts

This roadmap is an opinionated guide to learning Rust, created and designed by me, [Henrique Costa](https://github.com/HenriqueCosta05). It is intended to help me and others learn Rust in a structured way, starting from the basics and gradually moving towards more advanced concepts.

# Step 1: Learn the basics of Rust

## [The Rust Programming Language](https://rust-book.cs.brown.edu/) - Complete Index

### Core Chapters

- [1. Getting Started](./01_Fundamentals/ch_01/README.md)
  - [1.1. Installation](./01_Fundamentals/ch_01/README.md#installing-rust)
  - [1.2. Hello, World!](./01_Fundamentals/ch_01/README.md#the-anatomy-of-a-rust-program)
  - [1.3. Hello, Cargo!](./01_Fundamentals/ch_01/README.md#using-cargo)

- [2. Programming a Guessing Game](./01_Fundamentals/ch_02/README.md)

- [3. Common Programming Concepts](./01_Fundamentals/ch_03/README.md)
  - [3.1. Variables and Mutability](https://rust-book.cs.brown.edu/ch03-01-variables-and-mutability.html)
  - [3.2. Data Types](https://rust-book.cs.brown.edu/ch03-02-data-types.html)
  - [3.3. Functions](https://rust-book.cs.brown.edu/ch03-03-how-functions-work.html)
  - [3.4. Comments](https://rust-book.cs.brown.edu/ch03-04-comments.html)
  - [3.5. Control Flow](https://rust-book.cs.brown.edu/ch03-05-control-flow.html)

- [4. Understanding Ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html)
  - [4.1. What is Ownership?](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html)
  - [4.2. References and Borrowing](https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html)
  - [4.3. Fixing Ownership Errors](https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html)
  - [4.4. The Slice Type](https://rust-book.cs.brown.edu/ch04-04-slices.html)
  - [4.5. Ownership Recap](https://rust-book.cs.brown.edu/ch04-05-ownership-recap.html)

- [5. Using Structs to Structure Related Data](https://rust-book.cs.brown.edu/ch05-00-structs.html)
  - [5.1. Defining and Instantiating Structs](https://rust-book.cs.brown.edu/ch05-01-defining-structs.html)
  - [5.2. An Example Program Using Structs](https://rust-book.cs.brown.edu/ch05-02-example-structs.html)
  - [5.3. Methods](https://rust-book.cs.brown.edu/ch05-03-method-syntax.html)

- [6. Enums and Pattern Matching](https://rust-book.cs.brown.edu/ch06-00-enums.html)
  - [6.1. Defining an Enum](https://rust-book.cs.brown.edu/ch06-01-defining-an-enum.html)
  - [6.2. The match Control Flow Construct](https://rust-book.cs.brown.edu/ch06-02-match.html)
  - [6.3. Concise Control Flow with if let and let...else](https://rust-book.cs.brown.edu/ch06-03-if-let.html)
  - [6.4. Ownership Inventory #1](https://rust-book.cs.brown.edu/ch06-04-inventory.html)

### Intermediate Chapters

- [7. Packages, Crates, and Modules](https://rust-book.cs.brown.edu/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
  - [7.1. Packages and Crates](https://rust-book.cs.brown.edu/ch07-01-packages-and-crates.html)
  - [7.2. Control Scope and Privacy with Modules](https://rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html)
  - [7.3. Paths for Referring to an Item in the Module Tree](https://rust-book.cs.brown.edu/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
  - [7.4. Bringing Paths Into Scope with the use Keyword](https://rust-book.cs.brown.edu/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)
  - [7.5. Separating Modules into Different Files](https://rust-book.cs.brown.edu/ch07-05-separating-modules-into-different-files.html)

- [8. Common Collections](https://rust-book.cs.brown.edu/ch08-00-common-collections.html)
  - [8.1. Storing Lists of Values with Vectors](https://rust-book.cs.brown.edu/ch08-01-vectors.html)
  - [8.2. Storing UTF-8 Encoded Text with Strings](https://rust-book.cs.brown.edu/ch08-02-strings.html)
  - [8.3. Storing Keys with Associated Values in Hash Maps](https://rust-book.cs.brown.edu/ch08-03-hash-maps.html)
  - [8.4. Ownership Inventory #2](https://rust-book.cs.brown.edu/ch08-04-inventory.html)

- [9. Error Handling](https://rust-book.cs.brown.edu/ch09-00-error-handling.html)
  - [9.1. Unrecoverable Errors with panic!](https://rust-book.cs.brown.edu/ch09-01-unrecoverable-errors-with-panic.html)
  - [9.2. Recoverable Errors with Result](https://rust-book.cs.brown.edu/ch09-02-recoverable-errors-with-result.html)
  - [9.3. To panic! or Not to panic!](https://rust-book.cs.brown.edu/ch09-03-to-panic-or-not-to-panic.html)

- [10. Generic Types, Traits, and Lifetimes](https://rust-book.cs.brown.edu/ch10-00-generics.html)
  - [10.1. Generic Data Types](https://rust-book.cs.brown.edu/ch10-01-syntax.html)
  - [10.2. Defining Shared Behavior with Traits](https://rust-book.cs.brown.edu/ch10-02-traits.html)
  - [10.3. Validating References with Lifetimes](https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html)
  - [10.4. Ownership Inventory #3](https://rust-book.cs.brown.edu/ch10-04-inventory.html)

- [11. Writing Automated Tests](https://rust-book.cs.brown.edu/ch11-00-testing.html)
  - [11.1. How to Write Tests](https://rust-book.cs.brown.edu/ch11-01-writing-tests.html)
  - [11.2. Controlling How Tests Are Run](https://rust-book.cs.brown.edu/ch11-02-running-tests.html)
  - [11.3. Test Organization](https://rust-book.cs.brown.edu/ch11-03-test-organization.html)

- [12. An I/O Project: Building a Command Line Program](https://rust-book.cs.brown.edu/ch12-00-an-io-project.html)
  - [12.1. Accepting Command Line Arguments](https://rust-book.cs.brown.edu/ch12-01-accepting-command-line-arguments.html)
  - [12.2. Reading a File](https://rust-book.cs.brown.edu/ch12-02-reading-a-file.html)
  - [12.3. Refactoring to Improve Modularity and Error Handling](https://rust-book.cs.brown.edu/ch12-03-improving-error-handling-and-modularity.html)
  - [12.4. Adding Functionality with Test Driven Development](https://rust-book.cs.brown.edu/ch12-04-testing-the-librarys-functionality.html)
  - [12.5. Working with Environment Variables](https://rust-book.cs.brown.edu/ch12-05-working-with-environment-variables.html)
  - [12.6. Redirecting Errors to Standard Error](https://rust-book.cs.brown.edu/ch12-06-writing-to-stderr-instead-of-stdout.html)

- [13. Functional Language Features: Iterators and Closures](https://rust-book.cs.brown.edu/ch13-00-functional-features.html)
  - [13.1. Closures](https://rust-book.cs.brown.edu/ch13-01-closures.html)
  - [13.2. Processing a Series of Items with Iterators](https://rust-book.cs.brown.edu/ch13-02-iterators.html)
  - [13.3. Improving Our I/O Project](https://rust-book.cs.brown.edu/ch13-03-improving-our-io-project.html)
  - [13.4. Performance in Loops vs. Iterators](https://rust-book.cs.brown.edu/ch13-04-performance.html)

### Advanced Chapters

- [14. More about Cargo and Crates.io](https://rust-book.cs.brown.edu/ch14-00-more-about-cargo.html)
  - [14.1. Customizing Builds with Release Profiles](https://rust-book.cs.brown.edu/ch14-01-release-profiles.html)
  - [14.2. Publishing a Crate to Crates.io](https://rust-book.cs.brown.edu/ch14-02-publishing-to-crates-io.html)
  - [14.3. Cargo Workspaces](https://rust-book.cs.brown.edu/ch14-03-cargo-workspaces.html)
  - [14.4. Installing Binaries with cargo install](https://rust-book.cs.brown.edu/ch14-04-installing-binaries.html)
  - [14.5. Extending Cargo with Custom Commands](https://rust-book.cs.brown.edu/ch14-05-extending-cargo.html)

- [15. Smart Pointers](https://rust-book.cs.brown.edu/ch15-00-smart-pointers.html)
  - [15.1. Using Box<T> to Point to Data on the Heap](https://rust-book.cs.brown.edu/ch15-01-box.html)
  - [15.2. Treating Smart Pointers Like Regular References](https://rust-book.cs.brown.edu/ch15-02-deref.html)
  - [15.3. Running Code on Cleanup with the Drop Trait](https://rust-book.cs.brown.edu/ch15-03-drop.html)
  - [15.4. Rc<T>, the Reference Counted Smart Pointer](https://rust-book.cs.brown.edu/ch15-04-rc.html)
  - [15.5. RefCell<T> and the Interior Mutability Pattern](https://rust-book.cs.brown.edu/ch15-05-interior-mutability.html)
  - [15.6. Reference Cycles Can Leak Memory](https://rust-book.cs.brown.edu/ch15-06-reference-cycles.html)

- [16. Fearless Concurrency](https://rust-book.cs.brown.edu/ch16-00-concurrency.html)
  - [16.1. Using Threads to Run Code Simultaneously](https://rust-book.cs.brown.edu/ch16-01-threads.html)
  - [16.2. Transfer Data Between Threads with Message Passing](https://rust-book.cs.brown.edu/ch16-02-message-passing.html)
  - [16.3. Shared-State Concurrency](https://rust-book.cs.brown.edu/ch16-03-shared-state.html)
  - [16.4. Extensible Concurrency with Send and Sync](https://rust-book.cs.brown.edu/ch16-04-extensible-concurrency-sync-and-send.html)

- [17. Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams](https://rust-book.cs.brown.edu/ch17-00-async-await.html)
  - [17.1. Futures and the Async Syntax](https://rust-book.cs.brown.edu/ch17-01-futures-and-syntax.html)
  - [17.2. Applying Concurrency with Async](https://rust-book.cs.brown.edu/ch17-02-concurrency-with-async.html)
  - [17.3. Working With Any Number of Futures](https://rust-book.cs.brown.edu/ch17-03-more-futures.html)
  - [17.4. Streams: Futures in Sequence](https://rust-book.cs.brown.edu/ch17-04-streams.html)
  - [17.5. A Closer Look at the Traits for Async](https://rust-book.cs.brown.edu/ch17-05-traits-for-async.html)
  - [17.6. Futures, Tasks, and Threads](https://rust-book.cs.brown.edu/ch17-06-futures-tasks-threads.html)

- [18. Object Oriented Programming Features](https://rust-book.cs.brown.edu/ch18-00-oop.html)
  - [18.1. Characteristics of Object-Oriented Languages](https://rust-book.cs.brown.edu/ch18-01-what-is-oo.html)
  - [18.2. Using Trait Objects to Abstract over Shared Behavior](https://rust-book.cs.brown.edu/ch18-02-trait-objects.html)
  - [18.3. Implementing an Object-Oriented Design Pattern](https://rust-book.cs.brown.edu/ch18-03-oo-design-patterns.html)
  - [18.4. Ownership Inventory #4](https://rust-book.cs.brown.edu/ch18-04-inventory.html)
  - [18.5. Design Trade-offs](https://rust-book.cs.brown.edu/ch18-05-design-challenge.html)

- [19. Patterns and Matching](https://rust-book.cs.brown.edu/ch19-00-patterns.html)
  - [19.1. All the Places Patterns Can Be Used](https://rust-book.cs.brown.edu/ch19-01-all-the-places-for-patterns.html)
  - [19.2. Refutability: Whether a Pattern Might Fail to Match](https://rust-book.cs.brown.edu/ch19-02-refutability.html)
  - [19.3. Pattern Syntax](https://rust-book.cs.brown.edu/ch19-03-pattern-syntax.html)

- [20. Advanced Features](https://rust-book.cs.brown.edu/ch20-00-advanced-features.html)
  - [20.1. Unsafe Rust](https://rust-book.cs.brown.edu/ch20-01-unsafe-rust.html)
  - [20.2. Advanced Traits](https://rust-book.cs.brown.edu/ch20-02-advanced-traits.html)
  - [20.3. Advanced Types](https://rust-book.cs.brown.edu/ch20-03-advanced-types.html)
  - [20.4. Advanced Functions and Closures](https://rust-book.cs.brown.edu/ch20-04-advanced-functions-and-closures.html)
  - [20.5. Macros](https://rust-book.cs.brown.edu/ch20-05-macros.html)

- [21. Final Project: Building a Multithreaded Web Server](https://rust-book.cs.brown.edu/ch21-00-final-project-a-web-server.html)
  - [21.1. Building a Single-Threaded Web Server](https://rust-book.cs.brown.edu/ch21-01-single-threaded.html)
  - [21.2. From Single-Threaded to Multithreaded Server](https://rust-book.cs.brown.edu/ch21-02-multithreaded.html)
  - [21.3. Graceful Shutdown and Cleanup](https://rust-book.cs.brown.edu/ch21-03-graceful-shutdown-and-cleanup.html)
