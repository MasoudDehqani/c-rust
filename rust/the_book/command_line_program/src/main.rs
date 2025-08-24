/*
    grep -> globally search a regular expression and print

    - ripgrep is a Rusty version of grep (written in Rust)
*/

use std::env;

use command_line_program::{Config, mini_grep};

/*
    The if..let syntax is a syntactic sugar for pattern matching.
    if let PATTERN = EXPRESSION { ... } means: evaluate EXPRESSION, try to match it against PATTERN. if
    it matches run the block. if it doesn't, skip the block.

*/
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("{err}");
        std::process::exit(1);
    });

    let Config {
        query,
        file_path,
        is_case_sensitive,
    } = config;

    println!("searching for '{query}' in the file -> {file_path}");

    if let Err(e) = mini_grep(&query, &file_path, is_case_sensitive) {
        println!("{e}");
    };
}
