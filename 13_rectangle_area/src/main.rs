fn main() {
    let length = 50;
    let width = 30;

    println!(
        "Area of rectangle with length: {} and width: {} is equal to: {}",
        length, width, area(length, width)
    );

    let rect = (length, width);

    println!(
        "Area of rectangle with length: {} and width: {} is equal to: {}",
        rect.0, rect.1, area_with_tuple(rect)
    );

    let rect_struct = Rectangle{
        length,
        width
    };

    println!(
        "Area of rectangle {:?} is equal to: {}",
        rect_struct, rect_struct.area()
    );

    let other_rect_struct = Rectangle {
        length: 20,
        width: 20
    };

    println!(
        "Does {:?} can hold {:?}: {}",
        rect_struct, other_rect_struct,
        rect_struct.can_hold(&other_rect_struct))
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}
