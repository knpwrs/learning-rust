extern crate rand;
extern crate add;

pub fn add_two(num: i32) -> i32 {
    add::add(num, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_two() {
        assert_eq!(add_two(5), 7);
    }
}
