use session_one::fizzbuzz;

fn main() {
    let x = 5;
    let x = x + 1;

    println!("{}", x);

    let mut y = 0;

    for _ in 0..=5 {
        y += 1;
    }

    println!("{}", y);

    match grade(48) {
        Ok(g) => println!("Your grade is: {g}"),
        Err(e) => println!("Your grade is: {e}"),
    }

    println!("{}", fizzbuzz(60));
}

fn grade(n: i32) -> Result<char, String> {
    match n {
        0..=20 => Ok('E'),
        21..=40 => Ok('D'),
        41..=60 => Ok('C'),
        61..=80 => Ok('B'),
        81..=100 => Ok('A'),
        _ => Err("Out of range".to_string()),
    }
}
