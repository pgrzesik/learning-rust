use List::{Cons, Nil};

fn main() {
    let boxed = Box::new(5);
    println!("boxed = {}", boxed);

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
}

enum List {
    Cons(i32, Box<List>),
    Nil
}
