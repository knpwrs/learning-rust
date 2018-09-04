#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(5,
        Box::new(List::Cons(4,
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));
    println!("list = {:?}", list);
}
