use std::cmp::PartialOrd;
use std::fmt::Display;

trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        // Default implementation is optional
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
    fn summarize(&self) -> String {
        // Note that it isn’t possible to call the default implementation from an overriding
        // implementation of that same method. This is still useful for the template pattern.
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x = {} is the larger number", self.x);
        } else {
            println!("y = {} is the larger number", self.y);
        }
    }
}

fn main() {
    let news = NewsArticle {
        headline: String::from("Hello, World!"),
        location: String::from("Fooville"),
        author: String::from("knpwrs"),
        content: String::from("rabble rabble rabble"),
    };
    notify(&news);
    let tweet = Tweet {
        username: String::from("knpwrs"),
        content: String::from("RABBLE RABBLE RABBLE"),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());
    let list = vec![1,4,2,7,5,3,5];
    println!("{}", largest(&list));

    Pair::new(3, 4).cmp_display();
    Pair::new(5, 2).cmp_display();
}

fn notify<T: Summary>(item: &T) {
    println!("BREAKING NEWS! {}", item.summarize());
}

// Multiple trait bounds
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
// }

// Alternative syntax
// fn some_function<T, U>(t: T, u: U) -> i32
//   where T: Display + Clone,
//         U: Clone + Debug
// {
// }

// We can also implement traits for any types that implement other traits. The following would
// implement ToString for any type implementing Display (this is already in the stdlib):
// impl<T: Display> ToString for T {
// }
