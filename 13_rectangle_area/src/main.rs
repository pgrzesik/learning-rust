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
        "Area of rectangle with length: {} and width: {} is equal to: {}",
        rect_struct.length, rect_struct.width, area_with_struct(&rect_struct)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_with_struct(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}

struct Rectangle {
    length: u32,
    width: u32
}
