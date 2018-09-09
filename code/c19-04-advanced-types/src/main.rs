// Type aliases
type Kilometers = i32;
type Thunk = Box<Fn() + Send + 'static>;

// From the Write trait
// type Result<T> = Result<T, std::io::Error>;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 2;

    println!("{} + {} = {}", x, y, x + y);
}

fn takes_long_type(f: Thunk) {

}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("Foo!"))
}

// Workaround for unknown size for generic functions:
fn foo<T: ?Sized>(t: &T) { // must use a reference

}
