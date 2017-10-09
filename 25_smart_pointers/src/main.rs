use List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let boxed = Box::new(5);
    println!("boxed = {}", boxed);

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    let song = Mp3 {
        audio: vec![1, 2, 3, 4, 5],
        artist: Some(String::from("Travis Scott")),
        title: Some(String::from("RaRa"))
    };

    assert_eq!(*song, vec![1, 2, 3, 4, 5]);

    let smart = CustomSmartPointer {
        data: String::from("Data of smart pointer!")
    };
}

enum List {
    Cons(i32, Box<List>),
    Nil
}

struct Mp3 {
    audio: Vec<u8>,
    artist: Option<String>,
    title: Option<String>
}

impl Deref for Mp3 {
    type Target = Vec<u8>;

    fn deref(&self) -> &Vec<u8> {
        &self.audio
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping smart pointer with data: {}", &self.data);
    }
}
