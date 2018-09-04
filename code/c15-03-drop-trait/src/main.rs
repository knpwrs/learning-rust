struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping: `{}`", self.data);
    }
}

fn main() {
    let p = CustomSmartPointer { data: String::from("foobar") };
    let q = CustomSmartPointer { data: String::from("baz") };
    println!("{}", p.data);
    // In case we need to manually free memory we cannot call drop, we must use std::mem::drop
    std::mem::drop(p);
    println!("{}", q.data);
}
