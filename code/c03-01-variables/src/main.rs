fn main() {
    // Mutability
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    // Shadowing
    let y = 5;
    println!("The value of y is {}", x);
    let y = 6;
    println!("The value of y is {}", x);
}
