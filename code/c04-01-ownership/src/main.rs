fn main() {
    let x = 5;
    let y = 5;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("foobar");
    takes_ownership(s);
    // s is no longer valid because ownership has been given up
    // println!("{}", s);

    let z = 4;
    makes_copy(z);
    println!("{}", z);

    let s3 = gives_ownership(); // value moves to s3, ownership transfered
    println!("{}", s3);

    let s4 = String::from("banana");
    let s5 = gives_and_takes_back(s4);
    println!("{}", s5); // s4 is invalid here because ownership was ultimately transfered to s5

    // The ownership of a variable follows the same pattern every time: assigning a value to
    // another variable moves it. When a variable that includes data on the heap goes out of scope,
    // the value will be cleaned up by drop unless the data has been moved to be owned by another
    // variable.

    // Tuples can be returned to handle ownership transfer of multiple data, but...
    let (s6, len) = calculate_length(String::from("foo"));
    println!("{} is {} characters", s6, len);
}

fn takes_ownership(some_string: String) { // ownership taken
    println!("{}", some_string);
} // variable released

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("giving ownership");
    some_string // leaves scope
}

fn gives_and_takes_back(some_string: String) -> String {
    some_string
}

// ...there's got to be a better way! (see next file)
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
