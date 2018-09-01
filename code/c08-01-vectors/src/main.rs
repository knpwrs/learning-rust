enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let v: Vec<i32> = Vec::new();
    // Type inference
    // let v = vec![1, 2, 3];
    // Also type inference
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);

    if let Some(i) = v.get(2) {
        println!("{}", i);
    }

    // Panic!
    // let i = &v[3];
    // println!("{}", i);

    let first = &v[0];
    println!("{}", first);
    // Cannot push onto vector because it is also borrowed as immutable
    // This error is due to the way vectors work: adding a new element onto the end of the vector
    // might require allocating new memory and copying the old elements to the new space, if there
    // isn’t enough room to put all the elements next to each other where the vector currently is.
    // In that case, the reference to the first element would be pointing to deallocated memory.
    // The borrowing rules prevent programs from ending up in that situation.
    // v.push(7);

    // iteration
    let mut another_vector = vec![1,2,3];
    for i in &mut another_vector {
        // dereference and add
        *i += 100;
    }
    for i in another_vector {
        println!("{}", i);
    }

    // Storing different types on a vector via enums
    let cells = vec![
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Float(45.5),
        SpreadsheetCell::Text(String::from("foo")),
    ];
    for cell in cells {
        match cell {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}
