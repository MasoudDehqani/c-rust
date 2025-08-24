use std::error::Error;

mod utils;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub is_case_sensitive: bool,
}

/*
    - without using 'static lifetime specifier in the build function's signature, the compiler
    does not know where to borrow the &str (which is a borrowed type). There is no string related
    type in the params of the function and there is only a &str in the return type. So, when there is
    no related type in params, it will be defined in the function itself and by the end of the
    function execution, all defined variables will be deallocated. So, the compiler does not know
    where to borrow the &str in return type. By specifying 'static lifetime specifier, the compiler
    is informed that the string literal value would be in the binary which is available for the
    entire life of the program. In general, string literals are global and live forever. The 'static
    lifetime specifier also has the exact meaning

    - using 'mut' before the parameter, does not mean that the caller has to pass a mutable arg.
    it means, inside the function body, the binding named args can be mutated.
    - in Rust, mutability is not part of the type, it is part of the binding

    - Here Item is an associated type. How Item = String works here and how the associated type
    differs with generic type???

    - The reasons why Iterators exist in Rust are:
    composability
    laziness
    abstraction of iteration for all types of collections (having a general traversal pattern)
    zero-cost abstraction (converted to a kind of for loops at compile)
    extensibility (you can make your own iterator over anything)

    - if a type implements Iterator trait, you get all higher order functions (map, filter, enumerate, etc.)
    - if you use iterator, for example, over an array and make multiple transformations, no intermediate
    array created when chaining methods. Unless you consume an iterator, the code does not do anything due
    to laziness
*/
impl Config {
    pub fn build<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query is not provided"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("file path is not provided"),
        };

        let is_case_sensitive = std::env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            is_case_sensitive: !is_case_sensitive,
        })
    }
}

/*
    In general, 'dyn Error' means any error. (this Error here is std::error::Error)
    To be more precise, 'dyn Error' means it can be any error like
    std::io::Error or std::fmt::Error.
    Note that the std::error::Error is a trait, not a concrete type, but std::io::Error and
    std::fmt::Error are types (they are structs).

    The reason why 'dyn Error' is used inside a Box here is that the compiler needs to know
    the size at compile time and using 'dyn Error' without box here cause compiler error.

    Using ? at the end of read_to_string is for error propagation. In fact ? is a syntactic
    sugar or a shortcut for error propagation

    - Before Rust 2018 edition, using 'dyn' for specifying trait objects were optional

    - static dispatch - impl Trait
    - dynamic dispatch - dyn Trait

    - dispatch means: how does the program decide which function implementation to run when you call a method

    - static dispatch -> at compile time
    - dynamic dispatch -> at runtime

    - use generics when you want speed and compile time knowledge
    - use trait objects when you need flexibility and runtime polymorphism
*/
pub fn mini_grep(
    query: &str,
    file_path: &str,
    is_case_sensitive: bool,
) -> Result<(), Box<dyn Error>> {
    let file_content = std::fs::read_to_string(file_path)?;

    let search_result = match is_case_sensitive {
        true => utils::search::search_query_in_content(&file_content, query),
        false => utils::search::search_query_in_content_case_insensitive(&file_content, query),
    };

    println!("{:?}", search_result);

    Ok(())
}
