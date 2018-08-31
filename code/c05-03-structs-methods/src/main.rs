#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // We still need to use the & before self, just as we did in &Rectangle. Methods can take
    // ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as
    // they can any other parameter.
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.height < self.height
    }
    // Associated function, not a method! Namespaced functionality not bound to an instance.
    fn square(i: u32) -> Rectangle {
        Rectangle { width: i, height: i }
    }
}

// Multiple impl blocks are allowed, a use-case will be demonstrated in Chapter 10.

fn main() {
    let rect1 = Rectangle {
        width: 100,
        height: 120,
    };
    let rect2 = Rectangle {
        width: 50,
        height: 60,
    };
    let rect3 = Rectangle {
        width: 300,
        height: 400,
    };
    println!("area of rect1: {}", rect1.area());
    println!("rect 1 can hold rect 2: {}", rect1.can_hold(&rect2));
    println!("rect 1 can hold rect 3: {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(32);
    println!("area of rect4: {}", rect4.area());
}
