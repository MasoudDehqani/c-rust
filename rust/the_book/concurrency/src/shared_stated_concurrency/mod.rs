/*
  It seems MutexGuard which is the Ok() side of the Result type of lock() method return type
  on Mutex is a smart pointer and can be dereferenced using * operator

  - multiple ownership methods with Rc<T> and Arc<T>
  - Rc<T> -> a reference counted smart pointer (for using in single-threaded contexts)
  - Arc<T> -> atomically reference-counted (for using in multi-threaded contexts)
  - Arc<T> is thread-safe, means that it can safely used in multi-threaded contexts
  - thread safety comes with a performance penalty that you only want to pay when you really need to
  - Atomics are an additional kind of concurrency primitive
  - atomics work like primitive types but are safe to share across threads

  - counter is immutable, but we could get a mutable reference to value inside it; this means
  Mutex provides interior mutability as the Cell family does
  - Mutex<T> comes with the risk of creating deadlocks

  TODO: try creating a Rust program that has a deadlock; then research deadlock mitigation strategies for
  mutexes in any language and have a go at implementing them in Rust. The standard library API
  documentation for Mutex<T> and MutexGuard offers useful information

  TODO: read more about Sync and Send traits and how concurrency can be extensible in Rust
*/

use std::{
    // rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

pub fn share_state() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut counter_value = c.lock().unwrap();
            *counter_value += 1;
        });

        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", *counter.lock().unwrap());
}

/*
  STEP 4

  - using Arc::clone will only increment the reference count, and it doesn't make a deep copy of
  all the data like most types' implementations of clone do
*/
fn _share_state_step_4() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(h);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{}", *counter.lock().unwrap())
}

/*
  STEP 3

  Here in this code, using Rc<T> does not work because Rc<T> is not thread-safe. So, the compiler
  will throw an error saying that "Rc<Mutex<i32>> cannot be shared between threads safely". And also
  "the trait `Send` is not implemented for `Rc<Mutex<i32>>`"
*/
// fn _share_state_step_3() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..5 {
//         let counter = Rc::clone(&counter);
//         let h = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });

//         handles.push(h);
//     }

//     handles.into_iter().for_each(|h| h.join().unwrap());

//     println!("{}", *counter.lock().unwrap())
// }

/*
  STEP 2

  Here in this code, even with the use of "move" keyword for the closure passing to the spawn, the compiler
  throws an error saying that the counter "moved into closure here, in previous iteration of loop".
  Here we used move correctly, so the counter ownership is moved into the closure, but creating multiple
  threads in the loop requires multiple ownerships of the counter Mutex. So we need to use a tool which
  make it possible.
*/
// fn _share_state_step_2() {
//     let counter = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..5 {
//         let h = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });

//         handles.push(h);
//     }

//     for h in handles {
//         h.join().unwrap()
//     }

//     println!("{}", *counter.lock().unwrap())
// }

/*
  STEP 1

  mutex -> mutual exclusion
  - the mutex is described as guarding the data it holds via the locking system
*/
fn _share_state_step_1() {
    let n = Mutex::new(0);

    {
        let mut num = n.lock().unwrap();
        *num = 2;
    }

    println!("{n:?}");
}

fn _playground() {
    let counter = Mutex::new(0_i32);

    let s = counter.lock().unwrap();
    let _t: i32 = (*s).abs();
}
