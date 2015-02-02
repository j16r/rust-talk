struct Shape {
    x : i32,
    y : i32,
    points : Vec<i32>
}

fn get_x<'a>(&Shape {ref x, ..} : &'a Shape) -> &'a i32 {
    x
}

fn main() {
    let square = Shape{x: 0, y: 0, points: vec![0, 0, 1, 0, 1, 1, 1, 0]};
    println!("Shape's x is {}", get_x(&square));
}
