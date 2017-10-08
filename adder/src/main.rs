extern crate add_one;
extern crate add_two;

fn main() {
    let num = 1;
    println!("1 after adding one is {}", add_one::add_one(num));
    println!("1 after adding two is {}", add_two::add_two(num));
}
