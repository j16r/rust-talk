struct Shape {
    x : i32,
    y : i32,
    points : Vec<i32>
}

trait Drawable {
    fn draw(&self);
}

impl Drawable for Shape {
    fn draw(&self) {
        for point in self.points.iter() {
            // ...
        }
    }
}

fn main() {
    let square = Shape{x: 0, y: 0, points: vec![0, 0, 1, 0, 1, 1, 1, 0]};
    square.draw();
}
