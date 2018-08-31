fn main() {
    // let mut s = String::from("hello world");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("First word: {}", word);
    // String literals are slices
    println!("First word: {}", first_word("foo bar"));
    // s.clear(); // Cannot borrow s as mutable because it is also borrowed as immutable
}

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' { // if a space
//             return &s[..i] // return slice
//         }
//     }
//     &s[..] // the whole string is the first word
// }

// improved to also accept slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // if a space
            return &s[..i] // return slice
        }
    }
    &s[..] // the whole string is the first word
}
