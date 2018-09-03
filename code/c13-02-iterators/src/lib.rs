#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|shoe| shoe.size == shoe_size).collect()
}

struct Counter {
    value: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { value: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        if self.value < 6 {
            Some(self.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let sum: i32 = v1.iter().sum();
        assert_eq!(sum, 6);
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];
        // map is lazy
        let mapped: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(mapped, vec![2, 3, 4]);
    }

    #[test]
    fn iterator_filter() {
        let shoes = vec![
            Shoe { size: 12, style: String::from("red") },
            Shoe { size: 10, style: String::from("red") },
            Shoe { size: 12, style: String::from("green") },
            Shoe { size: 9, style: String::from("green") },
        ];
        let expected = vec![
            Shoe { size: 12, style: String::from("red") },
            Shoe { size: 12, style: String::from("green") },
        ];
        assert_eq!(shoes_in_my_size(shoes, 12), expected);
    }

    #[test]
    fn custom_iterator() {
        let counter = Counter::new();
        assert_eq!(counter.collect::<Vec<u32>>(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn other_iterator_methods() {
        // zip produces only four pairs; the theoretical fifth pair (5, None) is never produced
        // because zip returns None when either of its input iterators return None.
        let value: u32 = Counter::new().zip(Counter::new().skip(1))
            .map(|(x, y)| x * y)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(value, 18);
    }
}
