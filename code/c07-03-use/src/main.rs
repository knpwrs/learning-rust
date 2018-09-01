pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("Wow!");
            }
        }
    }
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Bring `of` into scope
use a::series::of;
// Bring `nested_modules` into scope
use a::series::of::nested_modules;
// Works for any namespace
use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn main() {
    of::nested_modules();
    nested_modules();
    println!("{:?} {:?}", Red, Yellow);
    println!("{:?}", Green);
}
