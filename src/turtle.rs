// This is a Rust program that simulates Logo-style turtle graphics!
// We'll build this step by step in our lessons

use std::f64::consts::PI;

// This struct represents our turtle
struct Turtle {
    x: f64,         // x position
    y: f64,         // y position
    angle: f64,     // direction (in radians)
    pen_down: bool, // is the pen down?
}

impl Turtle {
    // Create a new turtle at the center of the screen
    fn new() -> Turtle {
        Turtle {
            x: 0.0,
            y: 0.0,
            angle: 0.0,
            pen_down: true,
        }
    }
    
    // Move the turtle forward by a specific distance
    fn forward(&mut self, distance: f64) {
        let new_x = self.x + distance * self.angle.cos();
        let new_y = self.y + distance * self.angle.sin();
        
        if self.pen_down {
            println!("Drawing line from ({}, {}) to ({}, {})", 
                     self.x, self.y, new_x, new_y);
        } else {
            println!("Moving from ({}, {}) to ({}, {})", 
                     self.x, self.y, new_x, new_y);
        }
        
        self.x = new_x;
        self.y = new_y;
    }
    
    // Turn the turtle right by a specific angle in degrees
    fn right(&mut self, angle_degrees: f64) {
        let angle_radians = angle_degrees * PI / 180.0;
        self.angle -= angle_radians;
        println!("Turning right by {} degrees", angle_degrees);
    }
    
    // Turn the turtle left by a specific angle in degrees
    fn left(&mut self, angle_degrees: f64) {
        let angle_radians = angle_degrees * PI / 180.0;
        self.angle += angle_radians;
        println!("Turning left by {} degrees", angle_degrees);
    }
    
    // Lift the pen up (stop drawing)
    fn pen_up(&mut self) {
        self.pen_down = false;
        println!("Pen up");
    }
    
    // Put the pen down (start drawing)
    fn pen_down(&mut self) {
        self.pen_down = true;
        println!("Pen down");
    }
}

fn main() {
    println!("Let's draw a square with our Rust turtle!");
    
    // Create a new turtle
    let mut turtle = Turtle::new();
    
    // Draw a square
    for _ in 0..4 {
        turtle.forward(100.0);
        turtle.right(90.0);
    }
    
    println!("Square complete!");
}