fn main() {
    /*
        Session Two, task 1
    */
    let s1 = String::from("Hello");
    println!("{s1}");
    let s2 = s1;

    /*
        This line won't compile because of ownership rules.
        To be precise, String is an owned type and does not implement
        Copy trait (its data won't be implicitly copied) since its data
        owns memory on heap and copying it would be expensive.
        So s1 ownership moved to s2 and then after
        the move the s1 is used.
        There are some ways to fix:
        1. Not use s1 here and use it before move: code example on line 3
        2. Use a reference of s1 for assigning to s2:
        let s2 = &s1;
        Note that s2 here is not another string. It's a borrowed reference (pointer)
        to s1
        3. Use a clone of s1 assigning to s2
        let s2 = s1.clone();
        Now s2 is a deep copy of s1 and s1 and s2 have their own allocation by the
        allocator
    */
    // println!("{s1}");

    /*
        Session Two, task 2
    */
    let mut greeting = String::from("Hello");
    append_exclamation(&mut greeting);
    println!("{greeting}");

    let mut s = String::from("Rust");
    let r1 = &s;
    // println!("{r1}");
    // let r2 = &mut s;
    /*
        The line above produces compile error because a value cannot be borrowed both
        mutably and immutably at a time in a scope.
        Ways to fix and print both r1 and r2:
        1. print r1 before r2 definition
        2. create an inner scope and define r2 there and print it
    */
    {
        let r2 = &mut s;
        println!("{r2}");
    }

    println!("{r1}");
}

fn append_exclamation(s: &mut String) {
    *s += "!";
}
