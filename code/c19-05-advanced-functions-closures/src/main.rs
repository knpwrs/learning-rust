fn add_one(i: i32) -> i32 {
    i + 1
}

fn do_twice(f: fn(i32) -> i32, i: i32) -> i32 {
    f(i) + f(i)
}

fn returns_closure() -> Box<Fn(i32) -> i32> {
    Box::new(|x| x * 2)
}

fn main() {
    let answer = do_twice(add_one, 1);
    println!("(1 + 1) * 2 = {}", answer);

    let numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    println!("{:?}", list_of_strings);
    // same thing:
    let list_of_strings: Vec<String> = numbers
        .iter()
        .map(ToString::to_string)
        .collect();
    println!("{:?}", list_of_strings);

    let answer = returns_closure()(2);
    println!("Answer: {}", answer);
}
