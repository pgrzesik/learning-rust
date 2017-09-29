pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green
}

use a::series::of;
use TrafficLight::{Red, Green};
// Example of glob import
// use TrafficLight::*;

fn main() {
    of::nested_modules();
    let red = Red;
    let yellow = TrafficLight::Yellow;
    let green = Green;
}

