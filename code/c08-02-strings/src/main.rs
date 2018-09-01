fn main() {
    let mut mutable_string = String::new();
    // Any type that implements the Display trait has a to_string method
    let s = "initial contents".to_string();
    // UTF-8
    let hello = String::from("こんにちは");

    mutable_string.push_str(&hello[..]);
    mutable_string += ", ";
    let world = "World";
    mutable_string.push_str(world);
    mutable_string += "!";
    println!("{}", mutable_string);

    let mut lo = String::from("lo");
    lo.push('l');
    println!("{}", lo);

    let s1 = String::from("foo");
    let s2 = String::from("bar");
    // coerce &String to &str
    let s3 = s1 + &s2; // s1 moved here and cannot be used
    println!("{}", s2); // s2 still valid
    println!("{}", s3);

    // formatting
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // No ownership taken
    let tictactoe = format!("{}-{}-{}", tic, tac, toe);
    println!("{}", tictactoe);

    // Rust cannot index into strings, only slices are allowed
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    let another_hello = "नमस्ते";
    // The following iteration produces six characters
    for c in another_hello.chars() {
        println!("{}", c);
    }
    for b in another_hello.bytes() {
        println!("{}", b);
    }
    // Grapheme clusters are not provided by rust's standard library
}
