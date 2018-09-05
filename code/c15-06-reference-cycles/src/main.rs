use std::rc::{Rc, Weak};
use std::cell::RefCell;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

fn main() {
    // Example 1: Circular Reference
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc: {}", Rc::strong_count(&a));
    println!("a next item: {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc after b creation: {}", Rc::strong_count(&a));
    println!("b initial rc: {}", Rc::strong_count(&b));
    println!("b next item: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a: {}", Rc::strong_count(&b));
    println!("a rc count after changing a: {}", Rc::strong_count(&a));

    // Uncomment the following line to see that we have a cycle
    // STACK OVERFLOW!
    // println!("a next item: {:?}", a.tail());

    // The reference count of the Rc<List> instances in both a and b are 2 after we change the list
    // in a to point to b. At the end of main, Rust will try to drop b first, which will decrease
    // the count in each of the Rc<List> instances in a and b by 1.

    // However, because a is still referencing the Rc<List> that was in b, that Rc<List> has a
    // count of 1 rather than 0, so the memory the Rc<List> has on the heap won’t be dropped. The
    // memory will just sit there with a count of 1, forever.

    // Creating reference cycles is not easily done, but it’s not impossible either. If you have
    // RefCell<T> values that contain Rc<T> values or similar nested combinations of types with
    // interior mutability and reference counting, you must ensure that you don’t create cycles;
    // you can’t rely on Rust to catch them. Creating a reference cycle would be a logic bug in
    // your program that you should use automated tests, code reviews, and other software
    // development practices to minimize.

    // Another solution for avoiding reference cycles is reorganizing your data structures so that
    // some references express ownership and some references don’t. As a result, you can have
    // cycles made up of some ownership relationships and some non-ownership relationships, and
    // only the ownership relationships affect whether or not a value can be dropped.

    // Example 2: Circular weak references
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!("Leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());
        println!("Branch strong: {}, weak: {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
        println!("Leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    println!("Leaf strong: {}, weak: {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
