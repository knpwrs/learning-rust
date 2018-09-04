extern crate rand;
extern crate add_one;
extern crate add_two;

fn main() {
    let num = 10;
    println!("{} + 1 = {}", num, add_one::add_one(num));
    println!("{} + 2 = {}", num, add_two::add_two(num));
}
