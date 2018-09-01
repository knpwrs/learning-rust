use std::io;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        // match guard
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(file) => file,
                Err(err) => {
                    panic!("Could not create file: {:?}", err);
                }
            }
        },
        Err(err) => {
            panic!("Error opening file: {:?}", err);
        },
    };

    // The result type has some shortcuts on it
    let f2 = File::open("hello.txt").unwrap(); // Panics
    let f3 = File::open("hello.txt").expect("Could not open file."); // Panics with given message
}

// Error propagation
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = match File::open("hello.txt") {
//         Ok(file) => file,
//         Err(err) => return Err(err),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(err) => Err(err),
//     }
// }

// Shortcut for error propagation (`?` operator)
// There is a difference between what the match expression from Listing 9-6 and the ? operator do:
// error values used with ? go through the from function, defined in the From trait in the standard
// library, which is used to convert errors from one type into another. When the ? operator calls
// the from function, the error type received is converted into the error type defined in the
// return type of the current function. This is useful when a function returns one error type to
// represent all the ways a function might fail, even if parts might fail for many different
// reasons. As long as each error type implements the from function to define how to convert itself
// to the returned error type, the ? operator takes care of the conversion automatically.
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
