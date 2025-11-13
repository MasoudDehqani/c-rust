#![allow(dead_code)]
/*
  A thread pool allows you to process connections concurrently, increasing the throughput of your server
  - A traditional DoS attack originates from one computer or source, unlike a Distributed Denial-of-Service (DDoS) attack,
  which uses multiple compromised systems to flood the target

  - Making the server a multi-threaded one is just one way to improve the throughput of the server. Other
  options are: fork/join model - signle-threaded async I/O model - multithreaded async I/O model

  - test driven development
  - compiler driven development
*/
use std::{
    fs::read_to_string,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_request(&stream);
        });
    }

    /*
      This is for testing graceful shutdown
    */
    // for stream in listener.incoming().take(3) {
    //     let stream = stream.unwrap();
    //     pool.execute(move || {
    //         handle_request(&stream);
    //     });
    // }
}

type Job = dyn Fn() + 'static + Send;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Box<Job>>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message: Result<Box<dyn Fn() + Send + 'static>, mpsc::RecvError> =
                    receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => job(),
                    Err(_) => {
                        println!("Worker {id} disconnected");
                        break;
                    }
                }
            }
        });

        Self { id, thread }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Box<Job>>>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for i in 0..size {
            let receiver = Arc::clone(&receiver);
            let worker = Worker::new(i, receiver);

            workers.push(worker);
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    fn execute(&self, f: impl Fn() + 'static + Send) {
        self.sender.as_ref().unwrap().send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        // for worker in &mut self.workers {
        //     worker.thread.join().unwrap();
        // }

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

fn handle_request(mut stream: &TcpStream) {
    let buf = BufReader::new(stream);
    let request_line = buf.lines().next().unwrap().unwrap();

    let (content_path, status_line) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("hello.html", "HTTP/1.1 200 OK"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("hello.html", "HTTP/1.1 200 OK")
        }
        _ => ("404.html", "HTTP/1.1 404 NOT FOUND"),
    };

    let content = read_to_string(content_path).unwrap();
    let response = format!(
        "{status_line}\r\nContent-Length: {}\r\n\r\n{content}",
        content.len()
    );

    stream.write_all(&response.into_bytes()).unwrap();
}

/*
  Note: A saying you might hear about languages with strict compilers, such as Haskell and Rust, is “if the code compiles,
  it works.” But this saying is not universally true. Our project compiles up until stage 1, but it does absolutely nothing!
  If we were building a real, complete project, this would be a good time to start writing unit tests to check that the code
  compiles and has the behavior we want.
  TODO: unit test with TDD approach
  TODO: what would be different here if we were going to execute a future instead of a closure?
*/
// struct _ThreadPoolStage1;
// impl _ThreadPoolStage1 {
//     pub fn _new(_thread_count: u8) -> Self {
//         Self
//     }

//     pub fn _execute<F: FnOnce() + Send + 'static>(&self, _f: F) {}
// }

// fn _main_stage_1() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//     let pool = _ThreadPoolStage1::_new(4);

//     listener.incoming().for_each(|stream| {
//         let stream = stream.unwrap();
//         pool._execute(|| handle_connection(stream));
//     });
// }

// fn _multithreaded_server_infinite_threads() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

//     listener.incoming().for_each(|s| {
//         thread::spawn(|| _handle_connection_with_blocking(s.unwrap()));
//     });
// }

/*
  HTTP request structure:
  line 1: Method Request-URI HTTP-Version CRLF
  line 2: headers CRLF
  line 3: an empty line after all headers is the sign of end of headers
  line 4 and so on: message body

  HTTP response structure:
  line 1: HTTP-Version Status-Code Reason-Phrase CRLF
  line 2: headers CRLF
  line 3: an empty line after all headers is the sign of end of headers
  line 4 and so on: message body

  CRLF -> carriage return, line feed
  - using two CRLF means an end line and an empty line. that is why after headers "\r\n\r\n" is used. it means
  end of headers and then after that the body message can start
*/
// fn handle_connection(mut stream: TcpStream) {
//     let buf = BufReader::new(&stream);
//     let req_line = buf.lines().next().unwrap().unwrap();

//     let (path, status_line) = match req_line.as_str() {
//         "GET / HTTP/1.1" => ("hello.html", "HTTP/1.1 200 OK"),
//         _ => ("404.html", "HTTP/1.1 404 NOT FOUND"),
//     };

//     let content = fs::read_to_string(path).unwrap();
//     let content_length = content.len();
//     let res = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");

//     stream.write_all(res.as_bytes()).unwrap();
// }

/*
  It is a code to show how a single-threaded tcp server respond to requests when one of the
  requests requires a blocking operation
  - The code also is refactored
  - desugaring -> remove the syntactic sugar and expand the code to its original form
*/
// fn _handle_connection_with_blocking(mut stream: TcpStream) {
//     let buf = BufReader::new(&stream);
//     let req_line = buf.lines().next().unwrap().unwrap();

//     let (path, status_line) = match req_line.as_str() {
//         "GET / HTTP/1.1" => ("hello.html", "HTTP/1.1 200 OK"),
//         "GET /blocking HTTP/1.1" => {
//             thread::sleep(Duration::from_secs(5));
//             ("hello.html", "HTTP/1.1 200 OK")
//         }
//         _ => ("404.html", "HTTP/1.1 404 NOT FOUND"),
//     };

//     let content = fs::read_to_string(path).unwrap();
//     let content_length = content.len();
//     let res = format!("{status_line}\r\nContent-Length: {content_length}\r\n\r\n{content}");

//     stream.write_all(res.as_bytes()).unwrap();
// }

/*
  - the kernel receives the packet data from the NIC -> stores it in its own buffer
  - the program then create its own buffer -> data is copied into the program's buffer in user space
  - a buffer is just a chunk of memory where you temporarily store data while moving moving it
  between two places
  - NIC -> network interface controller (network interface card)
*/
// fn _handle_connection_with_buffer(mut stream: TcpStream) {
//     let buf = BufReader::new(&stream);
//     let req_line = buf.lines().next().unwrap().unwrap();

//     match req_line.as_str() {
//         "GET / HTTP/1.1" => {
//             let content = fs::read_to_string("hello.html").unwrap();
//             let content_length = content.len();
//             let content =
//                 format!("HTTP/1.1 200 OK\r\nContent-Length: {content_length}\r\n\r\n{content}");

//             stream.write_all(content.as_bytes()).unwrap();
//         }
//         _ => {
//             let content = fs::read_to_string("404.html").unwrap();
//             let content_length = content.len();
//             let content = format!(
//                 "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {content_length}\r\n\r\n{content}"
//             );

//             stream.write_all(content.as_bytes()).unwrap();
//         }
//     }
// }

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
// fn _handle_connection_with_read_to_string(mut stream: TcpStream) {
//     let mut s = String::new();
//     stream.read_to_string(&mut s).unwrap();

//     match s.lines().next() {
//         Some(status) => {
//             if status == "GET / HTTP/1.1" {
//                 let content = fs::read_to_string("hello.html").unwrap();
//                 let content_length = content.len();
//                 let res =
//                     format!("HTTP/1.1 200 OK\r\nContent-Length: {content_length}\r\n\r\n{content}");
//                 println!("{}", res);

//                 stream.write_all(res.as_bytes()).unwrap();
//             } else {
//                 let content = fs::read_to_string("404.html").unwrap();
//                 let content_length = content.len();
//                 let res = format!(
//                     "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {content_length}\r\n\r\n{content}"
//                 );

//                 stream.write_all(res.as_bytes()).unwrap();
//             }
//         }

//         None => println!("ERROR"),
//     };
// }

/*
  In response, after the status text in the first and only line, there are two CRLF which
  means there are no headers and no message body, because a two CRLF meand a line ending and
  an empty line which means end of status line and headers
  - In other words, the double CRLF marks end of headers and after that the message body starts, but
  if nothing follows, then there simply is no body
*/
// fn _handle_http_request(mut stream: TcpStream) {
//     let buf = BufReader::new(&stream);
//     let _http_request: Vec<_> = buf
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();

//     let response = "HTTP/1.1 200 OK\r\n\r\n";
//     stream.write_all(response.as_bytes()).unwrap();
// }

/*
  The comments with triple slash (///) are doc comments
  By running the "cargo doc --open" command, an html file will be opened, and
  there would be a section for ThreadPool struct which can be clicked then
  these explanations inside the doc comment will appear there

  worker -> In a worker pool implementation, a worker is a unit of execution responsible for
  performing a specific task from a job queue
*/

// struct ThreadPool;

// impl ThreadPool {
//     /// Create a new ThreadPool.
//     ///
//     /// The size is the number of threads in the pool.
//     ///
//     /// # Panics
//     ///
//     /// The `new` function will panic if the size is zero.
//     pub fn new(size: usize) -> Self {
//         assert!(size > 0);
//     }
// }

// TODO: change new associated function to "build" with this signature
// pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>
//
// pub struct ThreadPool;
// impl ThreadPool {
//     pub fn new(size: usize) -> ThreadPool {
//         assert!(size > 0);
//         ThreadPool
//     }

//     pub fn execute<F>(&self, f: F)
//     where
//         F: FnOnce() + Send + 'static,
//     {
//     }
// }
