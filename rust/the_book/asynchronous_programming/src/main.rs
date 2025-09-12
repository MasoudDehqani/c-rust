/*
    - communication over channel VS communication over static memory (if the data is read-only)
    - awaiting on a join handle is one way of communicate the outcome of the spawned operation
    back to the caller

    - ??? spawning in tokio -> spawning a future ???
    - spawning from std::thread -> spawning a thread
*/

fn main() {
    println!("Hello, world!");
}
