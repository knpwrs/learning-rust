fn main() {
    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    // unicode characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("unicode characters: {} {} {}", c, z, heart_eyed_cat);

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tuple: ({}, {}, {})", x, y, z);
    println!("tuple: ({}, {}, {})", tup.0, tup.1, tup.2);

    // arrays
    let arr = [1, 2, 3, 4, 5];
    println!("array: [{}, {}, {}, {}, {}]", arr[0], arr[1], arr[2], arr[3], arr[4]);
}
