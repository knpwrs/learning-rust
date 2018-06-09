fn main() {
    println!("Hello, world!");
    another_function({
        let x = 42;
        // expression
        x * 2
    }, {
        let x = 84;
        // expression
        x * 8
    });
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    let sum = add(x, y);
    println!("The sum is {}", sum);
    println!("Add one: {}", plus_one(sum));
}

fn add(x: i32, y: i32) -> i32 {
    // statement
    return x + y;
}

fn plus_one(x: i32) -> i32 {
    // expression
    x + 1
}
