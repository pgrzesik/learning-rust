fn main() {
    let some_vec = vec![1, 2, 3];
    let some_vec_iter = some_vec.iter();

    for val in some_vec_iter {
        println!("Value: {}", val);
    }
}
