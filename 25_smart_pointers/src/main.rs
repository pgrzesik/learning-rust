use List::{Cons, Nil};
use std::ops::Deref;
use std::mem::drop;
use std::cell::RefCell;

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
    drop(smart);
    println!("Whoops, already dropped!");

    let ref_cell_data = RefCell::new(5);
    demo(&ref_cell_data);
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

fn a_fn_that_borrows_immutably(a: &i32) {
    println!("a is {}", a);
}

fn a_fn_that_borrows_mutably(b: &mut i32) {
    *b += 1;
}

fn demo(r: &RefCell<i32>) {
    a_fn_that_borrows_immutably(&r.borrow());
    a_fn_that_borrows_mutably(&mut r.borrow_mut());
    a_fn_that_borrows_immutably(&r.borrow());
}