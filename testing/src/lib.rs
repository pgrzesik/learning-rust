#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub fn will_panic() {
    panic!("Panicked!");
}

pub fn greeting(name: &str) -> String{
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 8};
        let smaller = Rectangle { length:5, width: 5};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle { length: 8, width: 8};
        let smaller = Rectangle { length:5, width: 5};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("John");
        assert!(
            result.contains("John"),
            "Greeting did not contain John, value was `{}`", result
        );
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        will_panic();
    }
}
