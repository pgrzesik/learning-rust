fn main() {
    let some_vec = vec![1, 2, 3];
    let some_vec_iter = some_vec.iter();

    for val in some_vec_iter {
        println!("Value: {}", val);
    }

    let some_other_vec: Vec<_> = some_vec.iter().map(|x| x + 1).collect();
    println!("{:?}", some_other_vec);

    let shoes = vec![
        Shoe { size: 40, style: String::from("sandal") },
        Shoe { size: 42, style: String::from("sneaker") }
    ];

    let in_my_size = shoes_in_my_size(shoes, 42);

    for val in in_my_size {
        println!("In my size! {:?}", val);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}
