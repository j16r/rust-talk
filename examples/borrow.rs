fn increment(value: &mut i32)  {
    *value += 1;
}

fn main() {
    let mut value = 10;
    increment(&mut value);
    println!("Value after increment: {}", value);
    increment(&mut value);
    println!("Value after increment: {}", value);
}
