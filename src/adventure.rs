// Text Adventure Game
// This demonstrates using enums, structs, and handling more complex structures

use std::io::{self, Write};
use std::collections::HashMap;

// Define game locations as an enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Location {
    Home,
    Forest,
    Cave,
    Mountain,
    Lake,
    Village,
}

// Define available directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Game state structure
struct GameState {
    current_location: Location,
    inventory: Vec<String>,
    visited_locations: Vec<Location>,
    location_descriptions: HashMap<Location, String>,
    location_connections: HashMap<Location, HashMap<Direction, Location>>,
    found_items: HashMap<Location, Vec<String>>,
    game_finished: bool,
}

impl GameState {
    // Create a new game state
    fn new() -> Self {
        let mut location_descriptions = HashMap::new();
        location_descriptions.insert(
            Location::Home,
            "You are in your cozy house. Through the window, you see a forest to the north and a village to the east.".to_string(),
        );
        location_descriptions.insert(
            Location::Forest,
            "You are in a dense forest. Trees tower above you, and you hear birds chirping.".to_string(),
        );
        location_descriptions.insert(
            Location::Cave,
            "You are in a dark cave. It's damp and cold, but you see a glint of something shiny deeper inside.".to_string(),
        );
        location_descriptions.insert(
            Location::Mountain,
            "You are on a tall mountain. You can see for miles around, and the air is crisp and clean.".to_string(),
        );
        location_descriptions.insert(
            Location::Lake,
            "You are at a serene lake. The water is crystal clear, and fish swim lazily below the surface.".to_string(),
        );
        location_descriptions.insert(
            Location::Village,
            "You are in a bustling village. People are going about their day, and there's a market in the center.".to_string(),
        );

        // Set up connections between locations
        let mut location_connections = HashMap::new();
        
        // Home connections
        let mut home_connections = HashMap::new();
        home_connections.insert(Direction::North, Location::Forest);
        home_connections.insert(Direction::East, Location::Village);
        location_connections.insert(Location::Home, home_connections);

        // Forest connections
        let mut forest_connections = HashMap::new();
        forest_connections.insert(Direction::South, Location::Home);
        forest_connections.insert(Direction::West, Location::Cave);
        forest_connections.insert(Direction::East, Location::Mountain);
        location_connections.insert(Location::Forest, forest_connections);

        // Cave connections
        let mut cave_connections = HashMap::new();
        cave_connections.insert(Direction::East, Location::Forest);
        location_connections.insert(Location::Cave, cave_connections);

        // Mountain connections
        let mut mountain_connections = HashMap::new();
        mountain_connections.insert(Direction::West, Location::Forest);
        mountain_connections.insert(Direction::South, Location::Lake);
        location_connections.insert(Location::Mountain, mountain_connections);

        // Lake connections
        let mut lake_connections = HashMap::new();
        lake_connections.insert(Direction::North, Location::Mountain);
        lake_connections.insert(Direction::West, Location::Village);
        location_connections.insert(Location::Lake, lake_connections);

        // Village connections
        let mut village_connections = HashMap::new();
        village_connections.insert(Direction::West, Location::Home);
        village_connections.insert(Direction::East, Location::Lake);
        location_connections.insert(Location::Village, village_connections);

        // Place items in locations
        let mut found_items = HashMap::new();
        found_items.insert(Location::Forest, vec!["stick".to_string(), "berries".to_string()]);
        found_items.insert(Location::Cave, vec!["treasure".to_string(), "mysterious key".to_string()]);
        found_items.insert(Location::Mountain, vec!["telescope".to_string()]);
        found_items.insert(Location::Lake, vec!["fishing rod".to_string(), "shiny stone".to_string()]);
        found_items.insert(Location::Village, vec!["map".to_string(), "bread".to_string()]);

        GameState {
            current_location: Location::Home,
            inventory: Vec::new(),
            visited_locations: vec![Location::Home],
            location_descriptions,
            location_connections,
            found_items,
            game_finished: false,
        }
    }

    // Display current location and available directions
    fn describe_current_location(&self) {
        println!("\n{}", self.location_descriptions.get(&self.current_location).unwrap());
        
        println!("\nYou can go:");
        let connections = self.location_connections.get(&self.current_location).unwrap();
        for (direction, _) in connections {
            println!("- {:?}", direction);
        }

        // Check if there are items here
        if let Some(items) = self.found_items.get(&self.current_location) {
            if !items.is_empty() {
                println!("\nYou see:");
                for item in items {
                    println!("- {}", item);
                }
            }
        }
    }

    // Move in a direction
    fn move_direction(&mut self, direction: Direction) -> bool {
        if let Some(connections) = self.location_connections.get(&self.current_location) {
            if let Some(new_location) = connections.get(&direction) {
                self.current_location = *new_location;
                
                // Add to visited locations if it's the first time
                if !self.visited_locations.contains(&new_location) {
                    self.visited_locations.push(*new_location);
                }
                
                return true;
            }
        }
        
        println!("You can't go that way!");
        false
    }

    // Take an item
    fn take_item(&mut self, item_name: &str) -> bool {
        if let Some(items) = self.found_items.get_mut(&self.current_location) {
            if let Some(pos) = items.iter().position(|i| i.to_lowercase() == item_name.to_lowercase()) {
                let item = items.remove(pos);
                self.inventory.push(item.clone());
                println!("You picked up: {}", item);
                return true;
            }
        }
        
        println!("There's no {} here!", item_name);
        false
    }

    // Display inventory
    fn show_inventory(&self) {
        if self.inventory.is_empty() {
            println!("Your inventory is empty.");
        } else {
            println!("\nYour inventory:");
            for item in &self.inventory {
                println!("- {}", item);
            }
        }
    }

    // Check if the game is complete
    fn check_completion(&mut self) {
        if self.inventory.contains(&"treasure".to_string()) && 
           self.inventory.contains(&"map".to_string()) && 
           self.inventory.contains(&"mysterious key".to_string()) {
            println!("\n🎉 Congratulations! You've collected all the key items and completed the adventure! 🎉");
            println!("You've explored {} out of 6 locations!", self.visited_locations.len());
            self.game_finished = true;
        }
    }
}

// Process user commands
fn process_command(command: &str, state: &mut GameState) -> bool {
    let command = command.trim().to_lowercase();
    
    if command == "quit" || command == "exit" {
        return false;
    }
    
    if command == "help" {
        display_help();
        return true;
    }
    
    if command == "look" {
        state.describe_current_location();
        return true;
    }
    
    if command == "inventory" || command == "inv" {
        state.show_inventory();
        return true;
    }
    
    if command.starts_with("go ") || command.starts_with("move ") {
        let direction = command.split_whitespace().nth(1).unwrap_or("");
        match direction {
            "north" => { state.move_direction(Direction::North); }
            "south" => { state.move_direction(Direction::South); }
            "east" => { state.move_direction(Direction::East); }
            "west" => { state.move_direction(Direction::West); }
            _ => println!("Unknown direction: {}", direction),
        }
        state.describe_current_location();
        return true;
    }
    
    if command.starts_with("take ") || command.starts_with("get ") {
        let item = command.splitn(2, ' ').nth(1).unwrap_or("");
        state.take_item(item);
        state.check_completion();
        return true;
    }
    
    println!("I don't understand that command. Type 'help' for a list of commands.");
    true
}

// Display help information
fn display_help() {
    println!("\nAvailable commands:");
    println!("- look: Look around your current location");
    println!("- go [direction]: Move in a direction (north, south, east, west)");
    println!("- take [item]: Pick up an item");
    println!("- inventory: Show your inventory");
    println!("- help: Show this help message");
    println!("- quit: Exit the game");
}

fn main() {
    println!("============================================");
    println!("Welcome to the Rust Text Adventure Game!");
    println!("============================================");
    println!("You are on a quest to find three sacred items: the treasure, the map, and the mysterious key.");
    println!("Type 'help' for a list of commands.");
    
    let mut game_state = GameState::new();
    game_state.describe_current_location();
    
    while !game_state.game_finished {
        print!("\n> ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        if !process_command(&input, &mut game_state) {
            println!("Thanks for playing!");
            break;
        }
    }
}