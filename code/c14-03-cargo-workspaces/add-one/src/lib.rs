extern crate rand;
extern crate add;

pub fn add_one(num: i32) -> i32 {
    add::add(num, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_one() {
        assert_eq!(add_one(5), 6);
    }
}
