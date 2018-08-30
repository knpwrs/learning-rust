fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);
    let mut s2 = s1.clone();
    change(&mut s2);
    println!("{}", s2);
    // let &mut s3 = s2;
    // let &mut s4 = s2; // We can only have one mutable reference to a particular pece of data in
    // a particular scope
    //
    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // A data race is similar to a race condition and happens when these three behaviors occur:
    // * Two or more pointers access the same data at the same time.
    // * At least one of the pointers is being used to write to the data.
    // * There’s no mechanism being used to synchronize access to the data.
    //
    // Data races cause undefined behavior and can be difficult to diagnose and fix when you’re
    // trying to track them down at runtime; Rust prevents this problem from happening because it
    // won’t even compile code with data races!
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope but does not have ownership of the String, so nothing happens

// This doesn't work! You cannot modify something borrowed.
// fn change(some_string: &String) {
//     some_string.push_str(", world!");
// }

// We need a mutable reference:
fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// Rust prevents dangling pointers
// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello");
//     &s // return a reference to the String, s
// } // s goes out of scope and is dropped. Its memory goes away.
// // Welcome to the danger zone!
// Instead we can return the String directly and ownership will be moved out
