/*
    grep -> globally search a regular expression and print

    - ripgrep is a Rusty version of grep (written in Rust)
*/

use std::env;
use std::io::ErrorKind;

mod io_reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("searching for '{query}' in the file -> {file_path}");

    let file_content = io_reader::utils::reader(file_path);

    match &file_content {
        Ok(s) => {
            io_reader::utils::handle_content_in_query(&s, &query);
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("Not Found"),
            _ => println!("NOTHING"),
        },
    };
}
