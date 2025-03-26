use macroquad::prelude::*;
use tortue::prelude::*;

#[macroquad::main("Tortue")]
async fn main() {
    let mut turtle = Tortue::new();
    let mut current_shape = 1;
    let mut shape_drawn = false;

    loop {
        // Handle keyboard input
        if is_key_pressed(KeyCode::Key1) {
            current_shape = 1;
            shape_drawn = false;
            turtle.clear();
        }
        if is_key_pressed(KeyCode::Key2) {
            current_shape = 2;
            shape_drawn = false;
            turtle.clear();
        }
        if is_key_pressed(KeyCode::Key3) {
            current_shape = 3;
            shape_drawn = false;
            turtle.clear();
        }
        if is_key_pressed(KeyCode::Key4) {
            current_shape = 4;
            shape_drawn = false;
            turtle.clear();
        }

        // Draw the shape if not already drawn
        if !shape_drawn {
            match current_shape {
                1 => {
                    turtle.set_pen_color(RED);
                    for _ in 0..4 {
                        turtle.forward(100.0);
                        turtle.right(90.0);
                    }
                    shape_drawn = true;
                }
                2 => {
                    turtle.set_pen_color(GREEN);
                    for _ in 0..3 {
                        turtle.forward(100.0);
                        turtle.right(120.0);
                    }
                    shape_drawn = true;
                }
                3 => {
                    turtle.set_pen_color(BLUE);
                    for _ in 0..360 {
                        turtle.forward(2.0);
                        turtle.right(1.0);
                    }
                    shape_drawn = true;
                }
                _ => {
                    turtle.set_pen_color(PURPLE);
                    for _ in 0..8 {
                        turtle.forward(50.0);
                        turtle.right(45.0);
                    }
                    shape_drawn = true;
                }
            }
        }

        // Update and draw the turtle
        turtle.update();

        // Display instructions
        draw_text("Press 1: Square", 10.0, 20.0, 20.0, BLACK);
        draw_text("Press 2: Triangle", 10.0, 40.0, 20.0, BLACK);
        draw_text("Press 3: Circle", 10.0, 60.0, 20.0, BLACK);
        draw_text("Press 4: Octagon", 10.0, 80.0, 20.0, BLACK);

        next_frame().await
    }
}
