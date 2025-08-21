/*
    grep -> globally search a regular expression and print

    - ripgrep is a Rusty version of grep (written in Rust)
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
