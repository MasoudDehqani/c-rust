/*
    grep -> globally search a regular expression and print

    - ripgrep is a Rusty version of grep (written in Rust)
*/

use std::env;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("searching for '{query}' in the file -> {file_path}");

    let file_content = fs::read_to_string(file_path);

    match &file_content {
        Ok(s) => {
            let found = s.find(query);

            match found {
                Some(u) => {
                    let (_, second) = s.split_at(u);
                    let target = get_first_word(second);
                    println!("found '{target}' at {u}");
                }
                None => println!("Not Found"),
            }
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => println!("Not Found"),
            _ => println!("NOTHING"),
        },
    };
}

fn get_first_word(s: &str) -> &str {
    for (i, b) in s.bytes().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
