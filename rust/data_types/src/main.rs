use std::mem::size_of;

fn main() {
    /*
        Data types in Rust are either: Scalar or Compound
        Scalar data types are: interger - floating-point - boolean - char
        Compound types: Only two "primitive compound types" are introduced in "The Book":
        array - tuple
    */

    // Integer variables
    let reg_int = 3; // the type inferred. in rust, default inferred integer values are i32.
    let second_int: u32 = 8;
    let third_int: isize = 89;
    let fourth_int: i8 = -128;
    let fifth_int: u8 = 255;
    // integer overflow when compiled in release mode and compile error in debug mode
    // let fifth_int: u8 = 256;

    // these lines are examples of handling integer overflow
    let fifth_int_t: u8 = 255;
    let _t: u8 = fifth_int_t.wrapping_add(0);
    let _t: Option<u8> = fifth_int_t.checked_add(0);
    let _t: (u8, bool) = fifth_int_t.overflowing_add(0);
    let _t: u8 = fifth_int_t.saturating_add(0);

    // All integer types: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    /*
        Maximum and Minimum size of signed integer and float types are calculated like this (n = bits):
            min: 2 ** (n - 1) , max: (2 ** (n - 1)) - 1

        Maximum size of unsigned integer and float types are calculated like this (n = bits):
            (2 ** n) - 1
            Note that min is 0 in unsigned integers
    */

    // Different representation of integer
    let decimal_int = 2;
    let hex_int = 0x12;
    let oct_int = 0o7;
    let binary_int = 0b1011;
    let byte_int: u8 = b'A'; // only u8 is possible for this type

    // How to check size of types using standard library
    let size_of_isize_type = size_of::<isize>();

    println!("byte_int: {byte_int}");
    println!("sizeof isize_type: {size_of_isize_type}");
    println!(
        "reg_int: {reg_int}, second_int: {second_int}, third_int: {third_int}, fourth_int: {fourth_int}, fifth_int: {fifth_int}"
    );
    println!(
        "decimal_int: {decimal_int}, hex_int: {hex_int}, oct_int: {oct_int}, binary_int: {binary_int}, byte_int: {byte_int}"
    );

    // Floating-point Data Types
    let f: f32 = 8.45;
    let f2: f64 = 102.31;
    let f3 = 189.7; // default type is f64 as on modern CPU's has the same speed as f32
    // Floating-point numbers are represented according to the IEEE-754 standard

    println!("{f}, {f2}, {f3}");

    // Numeric operations in Rust: addition - subtraction - division - multiplication - remainder
    // division
    let quotient = 56.7 / 32.2;
    let truncated = 5 / 3; // Results in 1
    println!("quotient: {quotient}, truncated: {truncated}");

    // The Character Type
    let ch = 'h';
    let ch2: char = 'A';
    let ch3 = '😻';
    /*
        Rust’s char type is four bytes in size and represents a Unicode scalar value,
        which means it can represent a lot more than just ASCII.
        Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
    */
    println!("{ch}, {ch2}, {ch3}");

    // Compound Types
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let x1 = tup.0;
    println!("{x}, {y}, {z}, {x1}");

    // Array
    let arr = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [6, 7, 8, 9, 10];
    let arr3 = [3; 5];
    let fi = arr[0];
    // let ni = arr[5]; the compiler throw error and says it will panic at runtime
    /*
        This is an example of Rust’s memory safety principles in action. In many low-level languages,
        this kind of check is not done, and when you provide an incorrect index, invalid memory
        can be accessed. Rust protects you against this kind of error by immediately exiting instead of
        allowing the memory access and continuing
    */
    println!("{:?}, {:#?}, {:?}, {fi}", arr, arr2, arr3);
    /*
        The tuple without any values has a special name, unit. This value and its corresponding type
        are both written () and represent an empty value or an empty return type. Expressions
        implicitly return the unit value if they don’t return any other value.
    */
}

/*
    In Rust, typeof is a reserved keyword but not a currently implemented feature for runtime type introspection
    in the same way it might exist in other languages. Rust's type system is primarily static, meaning types are
    determined at compile time. While there are crates like type_info that provide runtime type information,
    they are not part of the standard library's core functionality for general type querying
*/
