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

    - floating-point numbers are numbers with decimal points
    - Rust's types for floating-point numbers are: f32 and f64 which are 32 bits and
    64 bits in size respectively
    - the default is f64 because on modern CPUs it's roughly the same speed as f32 but
    is capable of more precision
    - all floating point types are signed
    - floating-point numbers are represented according to IEEE-754 standard
    - Institute of Electrical and Electronics Engineers


    Numeric Operations
    - division:
        let quotient = 56.7 / 32.2
        let truncated = 5 / 3 -> results in 1
        let truncated = -5 / 3 -> results in -1


    - booleans are 1 byte in size

    The character type
    - the char type is Rust's most primitive alphabetic type
    - Rust's char is 4 bytes in size and represent a Unicode scalar value which means
    it can represent more than just ASCII
    - Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF

    Scalar types can group multiple values into one type
    - Rust's twp primitive compound types: tuples and arrays
    - both tuple and array are fixed-length, their size should be known at compile time
    - tuple can contain values of different types
    - array's element all should be of same type
    - array is stored on the stack
    - Which one tuple is stored on? stack or heap???
    - an empty tuple is a special type called unit
    - the type and the value of unit type are represented by ()
    - unit represents an empty value or an empty return type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; -> destructuring
    let x = tup.0;
    let y = tup.1;

    let arr: [i32; 4] = [1, 2, 3, 4];
    let arr = [3; 5]; -> [3, 3, 3, 3, 3]
    let x = arr[0];


    Invalid array element access:
    - in Rust, once it is attempted to access an array's element with an index
    out of the array bound, the program panicked at runtime to prevent an
    invalid access
    - when, in code, you explicitly tries to access an out-of-bound index from
    an array, the compiler would give an error and says this operation will panic
    at runtime
    - but if the index with which is tried to access an element in an array is
    not specified, the program will panic at runtime to prevent an invalid access
    to memory. it is one of Rust's memory safety principle in action. some other
    languages allow the invalid access and continue running the program which
    results in vulnerabilities and possible breach of security


    Functions
    - the main function is the entry point to many programs is Rust
    - Rust code uses snake_case convention for naming variables and functions

    - Are declaration, definition, initialization in Rust like C???

    - parameter VS argument in function
    - parameters are special values that are part of the functions's signature
    - arguments are concrete values passed to the function

    - statements VS expressions
    - Rust is an expression-based language
    - in Rust, function definitions and variable creation are statements
    - statements do not return any value, so you cannot assign a let statement
    to another variable.
    let y = 6 statement does not return a value. this is different from what
    happens in languages like C where the assignment return the value of the
    assignment.
    C code example: int x = y = 6; -> both x and y will have the value 6

    - calling a function is an expression
    - calling a macro is an expression
    - a scope block created with curly brackets is an expression
    - expressions do not end with semicolon. adding semicolon to the end of an expression
    will turn it to statement


    Control Flow

    - if expression
    - else if
    - else
    - we can optionally include an else expression
    - using if else like ternary operator in other languages
    - if expressions allow you to branch your code depending on conditions
    - branch - arm
    - the condition part of if expression must be of type boolean
    - if the condition of if is not boolean, it would be an undefined situation in Rust
    - loops: loop, while, for
    - continue, break
    - a value can be placed after break to return from the loop
    - a loop can be labelled: 'loop_name: loop
    - both return and break cause a change in control flow, both can take an expression as
    argument, compiler treat them as having unit value when using without any expression
    - looping through a collection with for loop
    - using range in for loop
    - while loop is a conditional loop
*/

fn main() {
    let tup: (i32, f64, u8) = (458, 89.3, 2);
    let (x, y, z) = tup;
    let x = tup.0;
    let y = tup.1;

    let arr: [i32; 4] = [1, 2, 3, 4];
    let arr = [3; 5];
    println!("{:?}", arr);

    another_function();
}

// doesn't matter a function is defined before or after where it is used. the compiler can see it in the scope
fn another_function() {
    println!("another function");
}
