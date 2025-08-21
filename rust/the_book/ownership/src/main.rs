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
