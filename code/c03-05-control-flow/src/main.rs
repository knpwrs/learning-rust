fn main() {
    let number = 3;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Condition as expression in statement
    let condition = true;
    let mut number = if condition {
        5
    } else {
        6
    };
    while number > 0 {
        println!("Number: {}", number);
        number -= 1;
    }

    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("Loop value: {}", element);
    }

    for num in (1..4).rev() {
        println!("{}...", num);
    }
    println!("LIFTOFF!");
}
