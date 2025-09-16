/*
  The comments with triple slash (///) are doc comments
  By running the "cargo doc --open" command, an html file will be opened, and
  there would be a section for ThreadPool struct which can be clicked then
  these explanations inside the doc comment will appear there

  worker -> In a worker pool implementation, a worker is a unit of execution responsible for
  performing a specific task from a job queue
*/

use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver},
    },
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} is executing...");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {} disconnected, shutting down...", id);
                        break;
                    }
                }
            }
        });

        Self { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(rx));

        let mut workers = vec![];

        for i in 1..=size {
            let worker = Worker::new(i, Arc::clone(&receiver));
            workers.push(worker);
        }

        Self {
            workers,
            sender: Some(tx.clone()),
        }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

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
