// let val = 34; this line produces compile error
const VAL: i32 = 34; // but this line is legal

fn main() {
    let x = 3;
    // x = 4; this line produce compile error
    let mut y = 5;
    println!("{y}");

    y = 6;
    println!("{x}, {y}");

    // shadowing:
    let z = 8;
    println!("{z}");
    let z = 10;
    println!("{z}");

    const CONSTANT_VARIABLE: i32 = 45; // constants must be type annotated

    // as some evaluation can be done at compile time in rust, this is legal:
    const SECOND_CONT_VARIABLE: i32 = 45 * 9;
    println!("{CONSTANT_VARIABLE}");
    println!("{SECOND_CONT_VARIABLE}");

    println!("{VAL}");

    // in shdowing the types can be different
    let str = "   "; // type is &str
    let str = str.len(); // type is usize
    println!("{str}");

    let mut str2 = "   ";
    println!("{str2}");
    str2 = "    ";
    println!("{str2}");
    // produces compile error, because mutation of type of a mutable variable is impossible
    // str2 = str2.len();

    // In C, the assignment return the value of the assignment. That's not the case in Rust
    // let x = (let y = 6); produces compile error
}
