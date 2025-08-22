use std::fs;
use std::io::Error;

pub fn reader(file_path: &str) -> Result<String, Error> {
    fs::read_to_string(file_path)
}

pub fn handle_content_in_query(content: &str, query: &str) {
    let found = content.find(query);

    match found {
        Some(u) => {
            let (start, end) = (u, u + query.chars().count());
            let target = &content[start..end];
            println!("found '{target}' at {u}");
        }
        None => println!("Not Found"),
    }
}
