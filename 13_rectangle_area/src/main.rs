fn main() {
    let length = 50;
    let width = 30;

    println!(
        "Area of rectangle with length: {} and width: {} is equal to: {}",
        length, width, area(length, width)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}
