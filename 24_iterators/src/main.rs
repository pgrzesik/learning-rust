fn main() {
    let some_vec = vec![1, 2, 3];
    let some_vec_iter = some_vec.iter();

    for val in some_vec_iter {
        println!("Value: {}", val);
    }

    let some_other_vec: Vec<_> = some_vec.iter().map(|x| x + 1).collect();
    println!("{:?}", some_other_vec);
}
