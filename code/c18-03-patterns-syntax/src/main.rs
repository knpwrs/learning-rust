fn main() {
    let x = 1;
    let y = Some(10);
    let c = 'c';

    // Matching Literals
    match x {
        1 => println!("One!"),
        2 => println!("One!"),
        3 => println!("One!"),
        4 => println!("One!"),
        _ => println!("Some other number!"),
    };

    // Combination
    match y {
        Some(50) => println!("WOW! FIFTY!"),
        Some(y) => println!("Oh, some other number: {}", y),
        None => println!("No numbers here!"),
    };

    // Multiple patterns
    match x {
        1 | 10 => println!("One or ten!"),
        _ => println!("Something else."),
    }

    // Ranges
    match c {
        'a'...'m' => println!("First half of the alphabet!"),
        'n'...'z' => println!("Second half of the alphabet!"),
        _ => println!("wat"),
    }

    {
        // Destructuring structs
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 32, y: 48 };
        let Point { x, y } = p;
        // escaped curly braces
        println!("{{ x: {}, y: {} }}", x, y);
        match p {
            Point { x: 32, y } => println!("x is 32 and y is {}", y),
            _ => println!("Don't know how to handle that!"),
        };
        // Destructuring references
        let points = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 5 },
            Point { x: 10, y: -3 },
        ];
        let sum_of_squares: i32 = points
            .iter()
            // it actually seems like & is inferred here...
            .map(|&Point { x, y }| x * x + y * y)
            .sum();
        println!("Sum of squares: {}", sum_of_squares);
        // Destructuring structs and tuples
        let ((feet, inches), Point { x, y }) = ((50, 4), Point { x: 20, y: 45 });
        println!("{} feet, {} inches, ({}, {})", feet, inches, x, y);
    }

    {
        // Destructuring enums
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.")
            },
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and in the y direction {}",
                    x,
                    y
                );
            }
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r,
                    g,
                    b
                )
            }
        }
    }

    {
        // Ignoring with _
        let mut setting_value = Some(10);
        let new_setting_value = Some(5);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value.");
            },
            (_, x) => {
                setting_value = x;
            },
        }

        println!("Setting is: {:?}", setting_value);

        let values = (5, 2, 7, 8, 8);
        let (first, _, third, _, fifth) = values;
        println!("First: {}, Third: {}, Fifth: {}", first, third, fifth);
        // or
        match values {
            (first, _, third, _, fifth) => println!("First: {}, Third: {}, Fifth: {}", first, third, fifth),
        };
    }

    {
        // Ignoring variables which start with _
        // The following produces a warning:
        let x = 10;
        // This does not:
        let _y = 5;

        let some_string = Some(String::from("Hello, World!"));
        if let Some(_s) = some_string {
            println!("Found a string!");
        }
        // println!("{:?}", some_string); // some_string moved!
        let some_string = Some(String::from("Hello, World!"));
        if let Some(_) = some_string {
            println!("Found a string!");
        }
        println!("{:?}", some_string); // not moved, just completely ignored
    }

    {
        // Ignoring parts of structs
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }
        // .. is faster than having to use _ for all ignored properties
        let Point { x, .. } = Point { x: 32, y: 43, z: 54 };
        println!("x: {}", x);

        // Also works in ranges
        let numbers = (45, 34, 67, 34, 56, 78, 98, 54, 23);
        let (first, .., last) = numbers;
        println!("first: {}, last: {}", first, last);
    }

    {
        // Creating references
        let robot_name = Some(String::from("Bender"));
        match robot_name {
            Some(ref name) => println!("{}", name),
            None => (),
        };

        let mut robot_name = Some(String::from("Bender"));
        match robot_name {
            Some(ref mut name) => {
                *name = String::from("Robot Jones");
            },
            None => (),
        };
        println!("{:?}", robot_name);
    }

    {
        // Match guards
        let num = Some(4);
        match num {
            Some(x) if x < 5 => println!("Small number!"),
            Some(x) => println!("Another number: {}", x),
            None => (),
        };

        let num = 4;
        let y = true;

        match num {
            4 | 5 | 6 if y => println!("Wowee!"),
            _ => (),
        };
    }

    {
        // @ bindings
        // The at operator (@) lets us create a variable that holds a value at the same time we’re
        // testing that value to see whether it matches a pattern.
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello { id: id_var @ 1...5 } => {
                println!("{} is between 1 and 5 (inclusive)", id_var);
            },
            Message::Hello { id: 6...10 } => {
                println!("Another number is between 6 and 10");
            },
            _ => (),
        }
    }
}
