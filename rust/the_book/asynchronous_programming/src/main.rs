#[allow(unused_imports)]
use asynchronous_programming::{
    counter_with_task, get_one_title_and_print, race_between_two_title,
};
use tokio::{self};

#[tokio::main]
async fn main() {
    race_between_two_title().await;
}

// fn main() {
//     get_one_title_and_print();
//     counter_with_task();
// }
