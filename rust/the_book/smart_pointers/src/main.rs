/*
  - pointer is a general concept for a variable that contains an address in memory. this address
  refers to, or points at, some other data
  - reference is the most common kind of pointer in Rust
  - references don't have any special capabilities other than referring to data, and they have
  no overhead

  smart pointers, on the other hand, are data structures that act like a pointer but also have
  additional metadata and capabilities.
  - smart pointers originated in C++

  - while references only borrow data, in many cases smart pointers own the data they point to
  - String and Vec<T> are also smart pointers

  most common smart pointers in the standard library:
  Box<T>
  Rc<T> -> reference counted -> enables multiple ownerships -> it is only for single-threaded contexts
  Arc<T> -> atomically reference counted -> atoms can safely transferred between threads
  Ref<T> and RefMut<T>, accessed through RefCell<T>

  - atomics are a kind of concurrency primitives
  - atomics work like primitive types but are safe to share across threads
  - types provided by std::sync::atomic provide safe, concurrent, atomic access to promitive types
  - the reason why all primitive types are not atomic, and why all standard library types are not
  implemented to to use Arc<T> by default, is that thread safety comes with a performance penalty that
  you only want to pay when you really need it.

  - Rust has variety of smart pointers defined in standard library
  - you can also write your own smart pointer
  - smart pointers are usually implemented using structs and they implement the Deref and Drop traits

  - interior mutability pattern -> an immutable type exposes an API for mutating an interior value

  - reference cycle -> leak memory
*/

mod box_smart_pointer;

fn main() {
    println!("Hello, world!");
}
