/*
  Box<T>
  box is the most straightforward smart pointer in Rust whose type is writter Box<T>

  - boxes allow you to store data on the heap rather than the stack. what remaing on the stack is
  the pointer to the heap data
  - boxes don't have performance overhead, other than storing their data on the heap
  instead of on the stack

  boxes, most often, are used in these sutuations:
  1. you want to use a data that its size can't be known at compile time and you want to use it in a
  context in which the size must be known at compile time. example: writing recursive data types like
  singly lined list (cons list)
  2. you have a large amount of data and you want to transfer ownership and you don't want all the data
  to be copied. transferring ownership of a large amount of data can take a long time since the data is
  copied around the stack. by using Box<T>, the large data is stroed on the heap and only small amount of
  pointer data is copied around the stack, while the data it references stays in one place on the heap
  3. for trait objects: you want to own a value and you care only that its type implements a paticular
  trait rather than being of a specific type
*/
pub fn _box_smart_pointer() {
    println!("test");
}
