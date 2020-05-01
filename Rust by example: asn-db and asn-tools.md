# Rust by example: asn-db and asn-tools

## Agenda

* What are crates?
 * Library vs binary crate
 * Binary crates with library code
 * Using libraries with `cargo add`

* Compiling and running
 * `check` vs `build` vs `install`
 * Running tests

* Crates documentation
 * module level
 * item level
 * comments

* Imports
 * Modules in Rust
 * Visibility
 * Re-exports

* Global variables
 * `const` expressions
 * Literal string vs literal binary string
 * UTF-8 encoding of source files

* Type in Rust
 * Primitive types
 * `structs`, tuples and other product types
 * `enums` sum type

* Functions, methods and traits
 * Free functions
 * Methods associated with types
 * Implementing traits and default implementations

* Deriving trait implementations
 * Deriving `Debug` and `Clone` trait
 * Manual implementation of `PartialEq` and `Eq` derive
 * Deriving `Serialize` with `serde_derive` procedural macros

* Implementing custom error types
 * `Result` type
 * `Error` trait and implementation
 * `From` trait and implementation
 * `?` operator and de-sugaring

* Iterators
 * `Option` type
 * `.next()`
 * Iterator composability and zero-cost performance
 * Creating iterators with `IntoIterator` trait
 * Consuming iterators with `collect` and "turbo fish" type annotations

* CSV parsing
 * Builder pattern
 * `Read` trait and I/O in rust
 * Type parametrisation
 * Trait bounds and `impl`
 * Lifetime parametrisation

* Serialization
 * `Write` trait
 * Writing and reading data with `serde` crate

* Panics
 * Aborting and unwinding
 * Explicit `panic!()` and implicit panic with `[]` index operator
 * Panic safety of libraries

* Testing
 * Writing unit tests
 * Running tests
 * Note on parallel execution

* Command line applications
 * Multiple binaries
 * `main` function and it's return type
 * Accessing arguments and environment

* Parsing command line arguments with `StructOpt`
 * `StructOpt` and `clap` crates
 * Annotating arguments
 * Accessing values
 * Help message

* Input and output
 * Printing to `stdout` and `stderr`
 * Logging with `log` crate
 * Reading from `stdin`

* Dynamic types
 * References, `Sized` trait and stack
 * Heap allocations and `Box` reference
 * Trait objects and `dyn`
 * `vtable` and zero cost abstraction
