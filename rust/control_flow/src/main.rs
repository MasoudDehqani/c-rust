fn main() {
    let x = 900;
    let y = if x > 500 { "BIG" } else { "SMALL" };

    /*
        if's are expressions, but I don't know why there is no difference wether it is ended with
        semicolon or not. Semicolon at the end of an expression makes it a statement. But in the
        case of if, the only case it needs an ending semicolon is when it is preceded by a let
        statement on the left of the assignment. Otherwise, it does not need a semicolon
    */
    let arr = [1, 2, 3, 4, 5];

    for num in arr {
        println!("{}", num)
    }

    // let l = if x > 500 {
    //     println!("HICHI");
    // } else {
    //     ()
    // };

    println!("{y}");
}
