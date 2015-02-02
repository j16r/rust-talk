#[derive(Show)]
struct Spaceship {
  x : i32,
  y : i32,
  health : u8
}

fn main() {
  let player1 = Spaceship { x: 0, y : 0, health : 100 };
  println!("Player 1 is {:?}", player1);
  // Player 1 is Spaceship { x: 0, y: 0, health: 100 }
}
