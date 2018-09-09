use std::ops::Add;
use std::fmt;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Human {
    fn fly(&self) {
        println!("Flaps arms furiously.");
    }
}
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Wingardium Leviosa!");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Supertraits
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point { }

// Newtype pattern
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// The above is a new type. If we wanted the new type to have every method the inner type has,
// implementing the Deref trait (discussed in Chapter 15 in the “Treating Smart Pointers like
// Regular References with the Deref Trait” section) on the Wrapper to return the inner type would
// be a solution.

fn main() {
    // Operator overloading
    assert_eq!(Point { x: 32, y: 32 } + Point { x: 16, y: 16 }, Point { x: 48, y: 48 });
    assert_eq!(Millimeters(2500) + Meters(1), Millimeters(3500));

    // Disambiguating methods
    let p =  Human;
    p.fly();
    Pilot::fly(&p);
    Wizard::fly(&p);

    // Disambiguating associated functions
    println!("A baby dog is called a {}", Dog::baby_name()); // wrong!
    // println!("A baby dog is called a {}", Animal::baby_name()); // cannot work!
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // correct! We must use the fully-qualified name.

    let p = Point { x: 32, y: 45 };
    p.outline_print();

    let w = Wrapper(vec![String::from("Hello"), String::from("World!")]);
    println!("w = {}", w);
}
