// use std::cmp::PartialOrd;
// use std::fmt::Display;

pub trait Summary {
    fn summary(&self) -> String;
    fn summary_with_default(&self) -> String {
        String::from("This is a default summary")
    }
    fn summary_from_summary(&self) -> String {
        format!("summary from summary: {}", self.summary())
    }
}

// pub struct Pair<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }
//
// impl<T: PartialOrd + Display> Pair<T> {
//     fn cmp_display(&self) {
//         match self.x >= self.y {
//             true => println!("The largest member is x whose value is {}", self.x),
//             false => println!("The largest member is y whose value is {}", self.y),
//         }
//     }
// }

/*
    blanket implementation: conditionally implement a trait for any type that implementsanother
    another trait
*/
// impl<T: Display> ToString for T {
//     // --snip
// }
// let s = s.to_string();
