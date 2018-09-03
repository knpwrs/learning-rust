use std::fmt::Display;

// Restrict the returned lifetime to be the shorter of the two lifetimes
// Rust is unable to unfer this on its own
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
    let novel = String::from("Well, Prince, so Genoa and Lucca are now just family estates of the Buonapartes.");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    let prt = i.announce_and_return_part("Hello!");
    println!("{}", prt);
    let result2 = longest_with_an_announcement(string1.as_str(), string2.as_str(), "Foooooo!");
    println!("{}", result2);
}

fn longest_with_an_announcement<'a, T: Display>(a: &'a str, b: &'a str, ann: T) -> &'a str {
    println!("Announcment! {}", ann);
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
