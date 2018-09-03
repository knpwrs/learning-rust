#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(i: u32) -> u32 {
    i + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let large = Rectangle { width: 90, height: 80 };
        let small = Rectangle { width: 40, height: 45 };
        assert!(large.can_hold(&small));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let large = Rectangle { width: 90, height: 80 };
        let small = Rectangle { width: 40, height: 45 };
        assert!(!small.can_hold(&large));
    }

    #[test]
    fn adds_two() {
        assert_eq!(add_two(43), 45);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("keeen");
        assert!(
            result.contains("keeen"),
            "Greeting did not contain name: {}",
            result,
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(101);
    }
}
