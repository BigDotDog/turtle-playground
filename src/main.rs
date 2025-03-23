use std::io;

use turtle::Turtle;

fn main() {
    let mut turtle = Turtle::new();
    turtle.set_speed(25);

    println!("What shape would you like to draw?");
    println!("Type 1 for a square");
    println!("Type 2 for a triangle");
    println!("Type 3 for a circle");
    print!("Type here: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let shape: u32 = input.trim().parse().unwrap_or(1);
    match shape {
        1 => {
            for _ in 0..4 {
                turtle.forward(100.0);
                turtle.right(90.0);
            }
        }
        2 => {
            for _ in 0..3 {
                turtle.forward(100.0);
                turtle.right(120.0);
            }
        }
        3 => {
            for _ in 0..3 {
                turtle.forward(100.0);
                turtle.right(120.0);
            }
        }
        _ => {
            turtle.set_pen_color("purple");
            for _ in 0..8 {
                turtle.forward(50.0);
                turtle.right(45.0);
            }
        }
    }
}
