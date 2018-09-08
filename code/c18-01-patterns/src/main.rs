fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "27".parse();

    // Mixing `if let`, `else if`, `else if let`, and `else`.
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background.", color);
    } else if is_tuesday {
        println!("It's a green day!");
    } else if let Ok(age) = age {
        // age is shadowed for this scope
        // condition cannot be combined with previous `else if let`
        if age > 30 {
            println!("Using purple as the background color.");
        } else {
            println!("Using orange as the background color.");
        }
    } else {
        println!("Using blue as the background color.");
    }

    // Conditional loops
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(i) = stack.pop() {
        println!("Popped {} off the stack!", i);
    }

    // For loops
    let v = vec!['a', 'b', 'c'];
    for (index, val) in v.iter().enumerate() {
        println!("{} is at position {}", val, index);
    }

    // Let statements
    let (x, y, z) = ('a', 'b', 'c');
    println!("({}, {}, {})", x, y, z);
    // If the number of elements in the pattern doesn’t match the number of elements in the tuple,
    // the overall type won’t match and we’ll get a compiler error.
    // let (x, y) = (1, 2, 3);

    // and...
    print_coordinates(&(4, 5));
}

// Function parameters
fn print_coordinates((x, y): &(i32, i32)) {
    println!("({}, {})", x, y);
}
