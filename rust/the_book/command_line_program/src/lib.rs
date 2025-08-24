use std::error::Error;

mod utils;

pub struct Config {
    pub query: String,
    pub file_path: String,
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

        Ok(Config { query, file_path })
    }
}

/*
    In general, 'dyn Error' means any error. (this Error here is std::error::Error)
    To be more precise, 'dyn Error' means it can be any error like
    std::io::Error or std::fmt::Error.
    Note that the std::error::Error is a trait, not a concrete type, but std::io::Error and
    std::fmt::Error are types (they are structs).
*/
pub fn mini_grep(query: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_content = std::fs::read_to_string(file_path)?;

    let search_result = utils::search::search_query_in_content(&file_content, &query);
    println!("{:?}", search_result);

    Ok(())
}
