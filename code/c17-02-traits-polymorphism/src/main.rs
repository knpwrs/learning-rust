extern crate c17_02_traits_polymorphism;
use c17_02_traits_polymorphism::*;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw SelectBox with width {} and height {}", self.width, self.height);
        println!("Options:");
        for op in self.options.iter() {
            println!(" - {}", op);
        }
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 45,
                height: 15,
                options: vec![String::from("Foo"), String::from("Bar"), String::from("Baz")],
            }),
            Box::new(Button {
                width: 30,
                height: 15,
                label: String::from("Submit"),
            }),
        ],
    };
    screen.run();
}
