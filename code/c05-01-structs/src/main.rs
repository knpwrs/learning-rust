// Named properties can be better than tuples for some usecases
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Data in structs must be valid as long as the struct is valid. This requires lifetimes, a concept
// to be introduced later.
// struct BadUser {
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
//     active: bool,
// }

// Tuple structs are effectively named tuple types which cannot be interchanged
struct Point(u8, u8, u8);
struct Color(u8, u8, u8); // These are different types!
struct Unit(); // The unit struct

fn main() {
    // Fields for structs don't have to be in order
    // Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain
    // fields as mutable.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone123"),
        active: true,
        sign_in_count: 1,
    };
    println!("username: {}", user1.username);
    user1.email = String::from("foobar@foo.info");
    println!("new email: {}", user1.email);
    let user2 = build_user(String::from("asdf"), String::from("asdf@fdsa.org"));
    println!("another username: {}", user2.username);
    // Update syntax
    let user3 = User {
        username: String::from("foobar"),
        email: String::from("foobar@barfoo.info"),
        ..user2
    };
    println!("and another username: {}", user3.username);
    let origin = Point(0, 0, 0);
    print_point(origin);
    print_color(Color(255, 255, 255));
}

fn build_user(username: String, email: String) -> User {
    // Shorthand syntax
    User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    }
}

fn print_point(p: Point) {
    println!("(x: {}, y: {}, z: {})", p.0, p.1, p.2);
}

fn print_color(Color(r, g, b): Color) {
    println!("rgb({}, {}, {})", r, g, b);
}
