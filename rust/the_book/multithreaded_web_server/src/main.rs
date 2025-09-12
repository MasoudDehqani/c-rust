use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener
        .incoming()
        .for_each(|s| handle_connection(s.unwrap()));
}

fn handle_connection(mut stream: TcpStream) {
    let buf = BufReader::new(&stream);
    let req_line = buf.lines().next().unwrap().unwrap();

    let (path, status_line) = match req_line.as_str() {
        "GET / HTTP/1.1" => ("hello.html", "HTTP/1.1 200 OK"),
        _ => ("404.html", "HTTP/1.1 404 NOT FOUND"),
    };

    let content = fs::read_to_string(path).unwrap();
    let content_length = content.len();
    let res = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");

    stream.write_all(res.as_bytes()).unwrap();
}

/*
  It is a code to show how a single-threaded tcp server respond to requests when one of the
  requests requires a blocking operation
  - The code also is refactored
*/
fn _handle_connection_with_blocking(mut stream: TcpStream) {
    let buf = BufReader::new(&stream);
    let req_line = buf.lines().next().unwrap().unwrap();

    let (path, status_line) = match req_line.as_str() {
        "GET / HTTP/1.1" => ("hello.html", "HTTP/1.1 200 OK"),
        "GET /blocking HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("hello.html", "HTTP/1.1 200 OK")
        }
        _ => ("404.html", "HTTP/1.1 404 NOT FOUND"),
    };

    let content = fs::read_to_string(path).unwrap();
    let content_length = content.len();
    let res = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");

    stream.write_all(res.as_bytes()).unwrap();
}

/*
  - the kernel receives the packet data from the NIC -> stores it in its own buffer
  - the program then create its own buffer -> data is copied into the program's buffer in user space
  - a buffer is just a chunk of memory where you temporarily store data while moving moving it
  between two places
  - NIC -> network interface controller (network interface card)
*/
fn _handle_connection_with_buffer(mut stream: TcpStream) {
    let buf = BufReader::new(&stream);
    let req_line = buf.lines().next().unwrap().unwrap();

    match req_line.as_str() {
        "GET / HTTP/1.1" => {
            let content = fs::read_to_string("hello.html").unwrap();
            let content_length = content.len();
            let content =
                format!("HTTP/1.1 200 OK\r\nContent-Length: {content_length}\r\n\r\n{content}");

            stream.write_all(content.as_bytes()).unwrap();
        }
        _ => {
            let content = fs::read_to_string("404.html").unwrap();
            let content_length = content.len();
            let content = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {content_length}\r\n\r\n{content}"
            );

            stream.write_all(content.as_bytes()).unwrap();
        }
    }
}

/*
  This code makes the server wait for an EOF. Here the tcp stream in the keep-alive connection does not
  have an EOF
  - EOF -> end of file
  - end of file in a network connection means end of bytes
  - fs::read_to_string method wait for an EOF which is not available in a tcp keep-alive connection
  - here we need a buffer reader which reads up to the end of each buffers coming to the server
  - a buffer is just a chunk of memory where you temporarily store data while moving moving it
  between two places
*/
fn _handle_connection_with_read_to_string(mut stream: TcpStream) {
    let mut s = String::new();
    stream.read_to_string(&mut s).unwrap();

    match s.lines().next() {
        Some(status) => {
            if status == "GET / HTTP/1.1" {
                let content = fs::read_to_string("hello.html").unwrap();
                let content_length = content.len();
                let res =
                    format!("HTTP/1.1 200 OK\r\nContent-Length: {content_length}\r\n\r\n{content}");
                println!("{}", res);

                stream.write_all(res.as_bytes()).unwrap();
            } else {
                let content = fs::read_to_string("404.html").unwrap();
                let content_length = content.len();
                let res = format!(
                    "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {content_length}\r\n\r\n{content}"
                );

                stream.write_all(res.as_bytes()).unwrap();
            }
        }

        None => println!("ERROR"),
    };
}
