/*
  Here’s the idea in a slogan from the Go language documentation:
  “Do not communicate by sharing memory; instead, share memory by communicating.”
  - there are two approaches regarding the quote from the Go language documentation:
  1. communicating by sharing memory: when threads communicate by using shared memory (this is not recommended)
  2. share memory by communicating: when threads communicate with each other and regulate
  thir use of shared memory (for example using mutexes and locks)

  - channel
  - Rust standard library provides an implementation of channels
  - a channel has two halves -> a transmitter and a receiver
  - a channel is said to be closed if either the transmitter or the receiver half
  is dropped

  - mpsc -> multiple producers, single consumer
  - Rust standard library's implementation of channel is of type mpsc
  - note that there can be multiple transmitter but only one receiver on each
  channel
  - but there is no limit for that one receiver to call the recv() method since the
  method use only a reference to the receiver. there is also no limit for iterating
  over that one receiver, provided that the ownership of the receiver is not moved.
*/

use std::{sync::mpsc, thread};

pub fn pass_message() {
    let (tx, rx) = mpsc::channel();
    let tx_2 = tx.clone();

    thread::spawn(move || {
        let s = String::from("Hi!");
        tx.send(s).unwrap();
    })
    .join()
    .unwrap();

    thread::spawn(move || {
        let s = String::from("Hi from thread 2");
        tx_2.send(s).unwrap();
    })
    .join()
    .unwrap();

    let f = rx.recv().unwrap();
    let f_2 = rx.recv().unwrap();
    // this line cause the program panic. because there is only two transmitter and there cannot be a third receiver
    // let f_3 = rx.recv().unwrap();

    println!("OUTSIDE: {f}");
    println!("OUTSIDE 2: {f_2}");

    // println!("OUTSIDE 3: {f_3}");

    /*
      but the lines below don't cause panic. because for loop works on iterators and iterators properly
      iterate only to the end of its size not beyond that.
      - note that rx itself is not an iterator, but when using it (not a reference to it) in a for loop,
      Rust implicitly calls into_iter() method on it and this method takes ownership of its callee
    */
    for r in &rx {
        println!("{r}")
    }

    for r in &rx {
        println!("{r}")
    }

    more_channels();
}

fn more_channels() {
    let (tx, rx) = mpsc::channel();

    let _handle = thread::spawn(move || {
        let values: Vec<String> = vec!["message", "from", "the", "thread"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        for val in values {
            tx.send(val).unwrap();
        }
    });

    for s in rx {
        println!("GOT: {s}")
    }
}
