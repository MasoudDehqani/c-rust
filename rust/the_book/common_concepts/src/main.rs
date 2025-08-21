/*
    Variables and Mutability

    - values (variable) which are bind to a name or identifier are
    by default immutable in Rust
    - immutability contributes to safety and easy concurrency that Rust provides

    - by using 'mut' keyword in variable definition, readers of the code
    be informed that the variable's value can be changed by other parts of the code

    constants differ in these way with variables:
    - constants must be type annotated
    - variables are by default immutable, but constants are always immutable
    - constants can be defined in any scope including global scope
    - constants are valid for the entire time the program runs within the scope
    in which they have been declared
    - constants must be initialized by constant expression values, not the result of
    some expression value that can only be evaluated at runtime.
    - note that Rust can evaluate some limited sets of expressions at compile time. for
    example this can be evaluated by compiler and thus can be the value of a constant:
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    - Rust convention for naming constants is all uppercase with underscores between
    words

    Shadowing differs in these ways with re-assigning a mutable variable:
    - by shadowing the type can be different, but by re-assigning one cannot alter
    the type

    Data Types

    - every value in Rust is of a certain type
    - type inference
    - type annotation
    - statically typed language
    - knowing all types at compile time

    - Rust's data types can be categorized to: scalar and compound
    - there are 4 primary scalar data types: integer - floating-point numbers - boolean - char
    - there are 2 primitive compound data types: tuple - array

    - a scalar type represents a single value
    - compound types group multiple values into one type

    These are all types of integers Rust have:
    u8 - i8
    u16 - i16
    u32 - i32
    u64 - i64
    u128 - i128
    usize - isize -> architecture dependent

    range of signed integers -> -(2**n-1) to (2**n-1) - 1
    range of unsigned integers -> 0 to (2**n) - 1

    These are all possible integer literal representation in Rust:
    - decimal -> 98100
    - hex -> 0xff
    - octal -> 0o77
    - binary -> 0b1101
    - byte (only u8) -> b'A'

    integers can be separated by underscore for the sake of readability:
    98_100
    0b110_1101

    integers can also written this way for type designation: 57u8

    - integer type defaults to i32 in Rust

    - integer overflow
    - in debug mode, integer overflow will be checked and cause the program to
    panic at runtime
    - building in release mode with --release flag, integer overflow won't be
    checked and instead of panic, the two's complement wrapping occurs

    - methods for explicitly handle possibility of overflows:
    wrapping_*
    checked_*
    overflowing_*
    saturating_*
*/

fn main() {
    println!("Hello, world!");
}
