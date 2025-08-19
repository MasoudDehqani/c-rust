pub fn fizzbuzz(n: u32) -> String {
    match (n % 3 == 0, n % 5 == 0) {
        (true, true) => String::from("FizzBuzz"),
        (true, false) => String::from("Fizz"),
        (false, true) => String::from("Buzz"),
        _ => n.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!("FizzBuzz", fizzbuzz(60));
        assert_eq!("Fizz", fizzbuzz(39));
        assert_eq!("Buzz", fizzbuzz(55));
        assert_eq!("1", fizzbuzz(1));
        assert_eq!("FizzBuzz", fizzbuzz(1290));
    }
}
