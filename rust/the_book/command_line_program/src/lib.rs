use std::io::ErrorKind;

mod io_reader;

pub fn mini_grep(query: &str, file_path: &str) {
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
