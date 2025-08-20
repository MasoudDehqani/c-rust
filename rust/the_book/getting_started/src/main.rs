/*
    Installation of Rust using rustup:
    rustup is command-line tool for managing Rust versions and associated tools

    Rust needs a linker to work. Linker is a program Rust uses to join its compiled
    outputs into one file.
    a C compiler usually includes linker -> for example GCC or Clang
    a C compiler is also useful because some common Rust packages depend on C code and
    need a C compiler

    - local documentation: rustup doc

    rustc is the rust compiler which can be installed via rustup
    the convention for rust file names is to use underscore between words if the name
    contains more than one word

    by using this command:
    rustc main.rs
    outside a cargo project, an executable file is produced by the compiler which can
    be executed by:
    ./main

    on windows it is: .\main

    as the file is an executable file, regardless of the OS, it should work

    - rustfmt is a formatter for Rust
    - standard Rust distribution

    - the 'main' function is special in Rust: it is always the first code that runs in
    every executable Rust program
    - Rust macros are a way to write code that generates code to extend Rust syntax

    - compiling and running are separate steps in Rust
    - Rust is an ahead-of-time compiled language

    - cargo is Rust's build system and package manager
    - dependencies -> libraries that your code needs
    - cargo in installed automatically using rustup

    - cargo.toml is the config file in the root of a cargo project
    - TOML -> cargo configuration format file
    - TOML -> Tom's obvious, minimal language
    - [package] - [dependencies]
    - cargo expects the source files live in the src directory
    - the top level project directory is only for files like readme, configs, license info and
    anything else not related to your code

    - in Rust, packages of code are referred to as crates

    - executable files in a cargo project:
    in debug mode (default build in cargo) -> target/debug
    in release mode -> target/release

    cargo.lock file keeps track of the exact versions of dependencies in the project

    - cargo build --release
    - compile with optimization
*/

fn main() {
    println!("Hello, world!");
}
