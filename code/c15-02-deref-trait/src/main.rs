use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // We must return a reference. If the deref method returned the value directly instead of a
        // reference to the value, the value would be moved out of self. We don’t want to take
        // ownership of the inner value inside MyBox<T> in this case or in most cases where we use
        // the dereference operator.
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    // behind the scenes: *(y.deref())
    assert_eq!(*y, 5);

    let name = Box::new(String::from("foobar"));
    hello(&name);
    // or...
    hello(&(*name)[..]);
}

// deref coercion: this ends up being a double deref
// Rust will automatically deref arguments to match parameters
// This is resolved at compile-time. No run-time penalty.
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
