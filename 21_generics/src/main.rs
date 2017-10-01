fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let numbers = vec![1, 3, 2, 5, 0];

    let result = largest(&numbers);

    println!("The largest number is {}", result);
}
