fn main() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("Three!");
    } else {
        println!("Not three!");
    }

    let optional_u8_value = Some(8u8);
    if let Some(i) = optional_u8_value {
        println!("Unwrapped: {}", i);
    }
}
