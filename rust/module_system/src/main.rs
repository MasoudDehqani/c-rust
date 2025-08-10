mod first_module;
mod fourth_module;
mod second_module;
mod third_module;

use crate::first_module::inside_first_module::some_function;
use crate::fourth_module::inside_fourth_module::{b_module::fn_of_b_module, fourth_one};
use crate::second_module::inside_second_module::second_inner_module::s;
use crate::third_module::inside_third_module::some_other_func;

const SOME_CONSTANT: i32 = 3;
static STATIC: i32 = 32;

fn main() {
    some_function();
    s();
    some_other_func();
    fourth_one();
    println!("Hello, world!");
    println!("{SOME_CONSTANT}");
    println!("{STATIC}");
    fn_of_b_module();
}
