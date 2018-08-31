#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1: (u32, u32) = (30, 50);
    println!("rect 1 is {:?}", rect1);
    println!("rect 1 is {:#?}", rect1);
    print_area(area_tuples(&rect1));
    let rect2 = Rectangle {
        width: 120,
        height: 60,
    };
    println!("rect 2 is {:?}", rect2);
    println!("rect 2 is {:#?}", rect2);
    print_area(area_structs(&rect2));
}

fn print_area(area: u32) {
    println!(
        "The area of the rectangle is {} square pixels.",
        area,
    );
}

// Make sure to borrow! We don't need to own this data.
fn area_tuples((x, y): &(u32, u32)) -> u32 {
    x * y
}

// Make sure to borrow! We don't need to own this data.
fn area_structs(Rectangle { width, height }: &Rectangle) -> u32 {
    width * height
}
