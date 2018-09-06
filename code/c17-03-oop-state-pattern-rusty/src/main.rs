extern crate blog;
use blog::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    println!("Drafted!");

    let post = post.request_review();
    println!("Review requested!");

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("Approved!");
}
