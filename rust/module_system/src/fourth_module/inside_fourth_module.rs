pub fn fourth_one() {
    println!("44444444444444444");
}

mod a_module {
    pub fn fn_of_a_module() {
        println!("fn of a module");
    }
}

pub mod b_module {
    pub fn fn_of_b_module() {
        super::a_module::fn_of_a_module();
        // not possible
        // fourth_one();

        super::fourth_one();
    }
}
