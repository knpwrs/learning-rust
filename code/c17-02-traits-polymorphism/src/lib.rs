// When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the
// types that might be used with the code that is using trait objects, so it doesn’t know which
// method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the
// trait object to know which method to call. There is a runtime cost when this lookup happens that
// doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to
// inline a method’s code, which in turn prevents some optimizations.
//
// You can only make object-safe traits into trait objects. Some complex rules govern all the
// properties that make a trait object safe, but in practice, only two rules are relevant. A trait
// is object safe if all the methods defined in the trait have the following properties:
//
// The return type isn’t Self.  There are no generic type parameters.  The Self keyword is an alias
// for the type we’re implementing the traits or methods on. Trait objects must be object safe because
// once you’ve used a trait object, Rust no longer knows the concrete type that’s implementing that
// trait. If a trait method returns the concrete Self type, but a trait object forgets the exact type
// that Self is, there is no way the method can use the original concrete type. The same is true of
// generic type parameters that are filled in with concrete type parameters when the trait is used:
// the concrete types become part of the type that implements the trait. When the type is forgotten
// through the use of a trait object, there is no way to know what types to fill in the generic type
// parameters with.

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Draw button \"{}\" with width {} and height {}", self.label, self.width, self.height);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
