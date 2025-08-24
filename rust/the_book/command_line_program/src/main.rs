/*
    grep -> globally search a regular expression and print

    - ripgrep is a Rusty version of grep (written in Rust)
*/

use std::env;

use command_line_program::{Config, mini_grep};

/*
    How if..let structure works here???
*/
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("{err}");
        std::process::exit(1);
    });

    let Config { query, file_path } = &config;

    println!("searching for '{query}' in the file -> {file_path}");

    if let Err(e) = mini_grep(query, file_path) {
        println!("{e}");
    };
}
