/*
    - ownership
    - move
    - safety is the absence of undefined behavior
    - pointer
    - all programming languages have the concept of pointer, they only differ in how
    - Rust goal: include as few runtime check as possible in its output binary programs
    - a foundational goal of Rust is to ensure your program never have undefined behavior
    - a secondary goal of Rust is to prevent undefined behavior at compile time instead of runtime
    - to compile programs into efficient binaries that require as few runtime checks as possible
    they expose it to the programmers
    - ownership as a discipline for memory safety
    - Rust provides a particular way to think about memory
    - stack frame
    - stack frames are automatically managed by Rust. Once a function is called, a stack frame
    is allocated and by the end of the execution of the function the stack frame is deallocated
    - reference is a type of pointer
    - borrowing
    - variables live in the stack
    - boxes live in the heap
    - memory management, in general, is the process of allocating memory and deallocating memory
    - Rust does not allow manual memory management (does no allow manual deallocation). Rust does
    not have free() function like C programming language.
    - But what are Drop trait and std::mem::drop method???
    - deallocation - freeing - dropping
    - focusing on frame-only variables is for better understanding of safety in Rust, but in fact
    compiler may put the variable in the register instead of stack frame. it is the implementation
    details and should not affect the understanding of the concept of safety in Rust
    - A Box's owner manages deallocation
    - Box is only a pointer to heap memory
    - if a variable owns a box, when Rust deallocates the variable's frame, Rust deallocate the
    box's heap memory
    - only types that implement Copy trait can be copied implicitly
    - copying all kinds of value can be expensive
    - Rust uses pointer to access data without copying
    - pointer - pointee
    - owned pointer -> example: Box
    - non-owned pointer -> reference
    - references change variables' permissions
    - borrow checker
    - a value should outlive all of its references
    - lifetime of a value is from its definition to its last usage
    - memory management
    - heap management
    - pointer management
    - manual memory management

    - ownership at first introduced in a paper for something completely other than safety
    it is introduced for the issue of leakage of internals of some data structures
    then the concept used for memory safety in Rust

    - when a program tries to read a value and expects it to be boolean and since the value
    does not exist due to an invalid memory access, it would be an undefined behavior. the
    address program tries to access could be anything which in not known at all

    - heap data is not tied to a specific stack frame
    - one common way to make a pointer is to allocate memory in the heap
    - collections like vectors, String, and HashMap uses Box

    - pointing to an invalid memory by a pointer in not problematic, using the invalid
    pointer is the issue

    - variables cannot be used after being moved
    - cloning avoids moves

    - mutable reference disband all permissions from variables

    - move-only APIs can be inconvenient to use
    - ownership of a variable which is moved to a function can be returned again by
    the function. but using it is verbose

    - &str type also includes &String but &String does not include &str

    - references are non-owning pointers
    - boxes are owning pointers

    - Rust implicitly insert dereferences and references in certain cases
    - These implicit conversions work for multiple layers of pointers
    - You won't be probably see the dereference operator very often when you read
    Rust code, because of implicit dereferencing

    - aliasing (immutable reference)
    - mutation
    - Rust avoids simultaneous aliasing and mutation
    - aliasing is accessing the same data through different variables
    - aliasing by itself is harmless. but combined with mutation could be disaster

    - references change permissions on places
    - a place is whatever can be used on left-hand side of an assignment

    - ensuring the safety of references by borrow checker

    - variables have 4 kinds of permissions on their data:
    Read (R)
    Write (W)
    Own (O)
    Flow (F)

    - the borrow checker finds permission violations
    - mutable references provide unique and non-owning access to data
    - immutable references (shared references)
    - mutable references (unique references)

    - permissions returned at the end of references' lifetime
    - data must outlive all of its references
*/

fn main() {
    /*
        As in Rust a value can only have one ownership at a time, here the ownership of
        s1 moves to s2 and then s2 is being used. Here move occurs because String does not
        implement Copy trait. Rust also invalidates the first variable, instead of being
        called a shallow copy, it’s known as a move.
        A value of type String, stores this info of the String on the stack:
        the address of pointer, length, capacity
        But integer type implement Copy trait and when we bind int1 to int2, it is copied
        and not moved, so the code compiles successfully.
        As String type has no fixed size and not known at compile time and can grow later,
        it is stored on heap not stack.
        There are many situations in a program that more than one piece of code need to
        use a value.
    */
    // let s1 = String::new();
    // let s2 = s1;
    // println!("{s1}");

    // One way to make the code work, is to clone the s1 and accept its performance overhead.
    let s1 = String::new();
    let _s2 = s1.clone();
    println!("{s1}");

    let int1 = 4;
    let _int2 = int1;

    println!("{int1}");

    /*
        Here also the ownership of s1 is moved to get_ownership. So s1 is no longer owned s1
        and is dropped.
    */
    // let s1 = String::new();
    // get_ownership(s1);
    // println!("{s1}");

    // String type can be mutable
    let mut str = String::from("some string");
    str.push_str(", and some other");

    /*
        String and string literal both can be mutated by binding them to a mutable variable. But
        the Rust book says string literal is immutable and String is mutable. Why is that????
    */
    let mut str_literal = "some literal string";
    println!("{str_literal}");
    str_literal = "some other";
    println!("{str_literal}");

    /*
        Here some_string re-assigned by a new value and Rust call drop to free the original value's
        memory
    */
    let mut some_string = String::from("Hi");
    println!("{some_string}");
    some_string = String::from("Hi, there!");
    println!("{some_string}");

    let str = String::from("Hello");
    let (s, len) = give_ownership(str);
    println!("The length of '{s}' is: {len}");

    let str = String::from("Hello");
    let len = calculate_length(&s);
    println!("The length of '{str}' is: {len}");

    // Here an immutable reference to str passed to the function.
    // let str = String::from("Hi");
    // concat_str(&str);

    let mut str = String::from("Hi");
    push_to_str(&mut str);

    /*
        There cannot be two mutable borrowing from a value at a time in a scope.
        Note that it produces error since both s1 and s2 are used after both of
        borrowing.
    */
    // let mut str = String::from("Hello");
    // let s1 = &mut str;
    // let s2 = &mut str;
    //
    // println!("{s1}, {s2}");

    // This code compiles. Because s1 won't be used after second borrowing
    // In fact the lifetime of s1 finished before second borrowing
    let mut str = String::from("Hello");
    let s1 = &mut str;
    println!("{s1}");
    let s2 = &mut str;

    println!("{s2}");

    // Also there cannot be mutable and immutable borrowing at a time in a scope
    // let mut str = String::from("Hello");
    // let s1 = &mut str;
    // let s2 = &str;
    // println!("{s1}, {s2}");

    // This code also is OK
    let mut str = String::from("Hello");
    {
        let s1 = &mut str;
        println!("{s1}");
    }

    let s2 = &mut str;
    println!("{s2}");

    // let _reference_to_nothing = dangling_ref();

    /*
        Slice Types
        string slice
    */

    let str = String::from("Hello World");
    let first_word = first_word(&str);
    println!("first word is: {first_word}");

    let _s = get_f("TEST");

    // explicit and implicit dereferencing and referencing
    let x = Box::new(-5);
    let _x_abs = i32::abs(*x); // explicit dereference
    let _x_abs = x.abs(); // implicit dereference

    let x_ptr = &x;
    let _x_abs = i32::abs(**x_ptr); // explicit dereference (twice)
    let _x_abs = x_ptr.abs(); // implicit dereference (twice)

    let s = String::from("Hello");
    let _s_len = str::len(&s); // explicit reference
    let _s_len = s.len(); // implicit reference
}

fn get_f(s: &str) -> &str {
    &s[1..]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, b) in bytes.iter().enumerate() {
        if *b == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
    Dangling Reference (Dangling Pinter):
    This code is trying to return a reference to a value which is owned by the function
    and dropped at the end of the scope.
*/
// fn dangling_ref() -> &String {
//     let s = String::from("Hello");
//
//     &s
// }

/*
    But this code works. Although the scope finished, as the ownership is moved out, nothing
    is dropped and deallocated
*/
fn _no_dangling() -> String {
    let s = String::from("Hello");

    s
}

// fn concat_str(s: &String) {
//     s.push_str(", there!");
// }

fn push_to_str(s: &mut String) {
    s.push_str(", there!");
}
fn give_ownership(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

// This function is borrowing an immutable reference of a String
fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn get_ownership(s: String) {
//     println!("{s}");
// }

/*
    String - string literal - string slice
    double free error
    lifetime of a value is between the time of its creation and the time it's last used in a scope
    data race - race condition
    dangling pointer: freeing some memory while preserving a pointer to that memory
    In idiomatic Rust, functions do not take ownership of their arguments unless they need to
*/
