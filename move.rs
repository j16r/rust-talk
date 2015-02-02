fn draw_shape<T>(points: Vec<T>) {
    for point in points.iter() {
        // ...
    }
}

fn main() {
    let points = vec![1, 2, 3];
    draw_shape(points);
    println!("Points are: {:?}", points); // Use of moved value points
}
