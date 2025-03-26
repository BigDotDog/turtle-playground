use std::io;

use macroquad::color::BLUE;
use tortue::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    let mut turtle = Tortue::new();

    loop {
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
                for _ in 0..360 {
                    turtle.forward(2.0);
                    turtle.right(1.0);
                }
            }
            _ => {
                turtle.set_pen_color(BLUE);
                for _ in 0..8 {
                    turtle.forward(50.0);
                    turtle.right(45.0);
                }
            }
        }

        turtle.draw().await;
    }
}
