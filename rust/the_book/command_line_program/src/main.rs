/*
    grep -> globally search a regular expression and print

    - ripgrep is a Rusty version of grep (written in Rust)
*/

use std::env;

use command_line_program::mini_grep;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("searching for '{query}' in the file -> {file_path}");

    mini_grep(query, file_path);
}
