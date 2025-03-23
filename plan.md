# 12-Week Rust Learning Plan for My Child

## Preparation Week: Setting Up

**Activities:**

- Install Rust using [`rustup`](https://rustup.rs)
- Install Visual Studio Code with the Rust extension
- Create a GitHub account to save progress
- Review basic Logo concepts your child already knows

**Resources to Gather:**

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings/) exercises

## Week 1: Hello, Rust!

**Learning Goals:**

- Understand what Rust is and why it's useful
- Create and run a first Rust program
- Learn about variables and basic data types

**Activities:**

1. Introduction to Rust and its benefits
2. Create and run "Hello, World!" program
3. Explore variables and basic data types
4. Draw comparisons between Logo and Rust

**Exercises:**

1. Modify the "Hello, World!" program to print your name
2. Create variables for age, favourite colour, and a hobby
3. Print all variables with descriptive text

**Weekend Project:**
Build a simple "About Me" program that asks for input and responds with facts about the user.

## Week 2: Control Flow

**Learning Goals:**

- Use if/else statements
- Work with loops (for, while)
- Understand basic expressions

**Activities:**

1. Explore if/else statements with examples
2. Learn about loops and when to use each type
3. Compare Logo loops to Rust loops

**Exercises:**

1. Create a program that checks if a number is positive, negative, or zero
2. Write a loop that counts from 1 to 10
3. Create a "FizzBuzz" program (print Fizz for multiples of 3, Buzz for multiples of 5, and FizzBuzz for multiples of both)

**Weekend Project:**
Build a simple number guessing game (first version).

## Week 3: Functions

**Learning Goals:**

- Create and call functions
- Pass parameters and return values
- Understand function signatures

**Activities:**

1. Explore function definitions and calls
2. Learn about parameters and return types
3. Create helper functions for existing programs

**Exercises:**

1. Create a function that squares a number
2. Write a function that converts temperature between Celsius and Fahrenheit
3. Refactor the FizzBuzz program to use functions

**Weekend Project:**
Start building the Turtle drawing simulator (part 1) - creating the turtle and basic movement functions.

## Week 4: Tuples, Arrays, and Vectors

**Learning Goals:**

- Work with compound data types
- Understand when to use each data structure
- Learn about indexing and iteration

**Activities:**

1. Explore tuples for fixed collections of different types
2. Learn about arrays for fixed-size collections
3. Use vectors for dynamic collections

**Exercises:**

1. Create a tuple to store a (x,y) coordinate and access its components
2. Create an array of your favourite foods and print them
3. Build a shopping list using a vector

**Weekend Project:**
Enhance the Turtle drawing simulator (part 2) - adding functions to draw shapes.

## Week 5: Ownership (Part 1)

**Learning Goals:**

- Understand Rust's ownership system
- Learn about variables and scope
- Explore how memory works in Rust

**Activities:**

1. Introduction to ownership with visual examples
2. Explore how variables are created and destroyed
3. Learn about the "move" concept

**Exercises:**

1. Experiment with creating variables and moving values
2. Trace memory usage in simple programs
3. Identify ownership issues in code snippets

**Weekend Project:**
Create a simple text-based menu system for a restaurant.

## Week 6: Ownership (Part 2) - Borrowing

**Learning Goals:**

- Understand references and borrowing
- Learn about mutable and immutable references
- Prevent common errors with the borrow checker

**Activities:**

1. Introduction to references with &
2. Learn about mutable references with &mut
3. Explore the rules of borrowing

**Exercises:**

1. Refactor previous programs to use references
2. Create functions that borrow values instead of taking ownership
3. Fix broken code with borrowing issues

**Weekend Project:**
Build a simple address book program that stores names and phone numbers.

## Week 7: Structs and Methods

**Learning Goals:**

- Create custom data types with structs
- Implement methods for structs
- Organise related data and functionality

**Activities:**

1. Define structs to represent objects
2. Add methods to structs with impl
3. Create instances of structs

**Exercises:**

1. Create a Person struct with name, age, and favourite colour
2. Add methods to greet and describe the person
3. Create multiple Person instances and call their methods

**Weekend Project:**
Enhance the Turtle drawing simulator (part 3) - converting to a struct-based design with methods.

## Week 8: Enums and Pattern Matching

**Learning Goals:**

- Create and use enumerations
- Implement pattern matching with match
- Model different states and options

**Activities:**

1. Define enums for related values
2. Use match expressions to handle different cases
3. Explore the Option enum for handling absence

**Exercises:**

1. Create an enum for different shapes with associated values
2. Write a function that calculates area based on the shape type
3. Use the Option type to handle calculations that might fail

**Weekend Project:**
Build a simple game with different character types using enums.

## Week 9: Error Handling

**Learning Goals:**

- Understand Rust's approach to error handling
- Use Result for operations that might fail
- Handle errors gracefully

**Activities:**

1. Introduction to the Result type
2. Learn about panic! and when to use it
3. Explore error propagation with the ? operator

**Exercises:**

1. Create functions that return Result values
2. Handle file operations with proper error checking
3. Refactor previous code to use better error handling

**Weekend Project:**
Create a simple file manager that can create, read, and write files with proper error handling.

## Week 10: Collections and Iterators

**Learning Goals:**

- Work with more advanced collections
- Use iterators to process collections
- Apply functional programming concepts

**Activities:**

1. Explore HashMap and HashSet
2. Learn about iterators and their methods
3. Use closures with iterators

**Exercises:**

1. Create a program that counts word frequency using a HashMap
2. Transform collections using map, filter, and collect
3. Create a custom iterator for a collection

**Weekend Project:**
Build a simple contact management system with search capabilities.

## Week 11: Traits and Generics

**Learning Goals:**

- Understand shared behaviour with traits
- Create flexible code with generics
- Combine traits and generics

**Activities:**

1. Define and implement traits
2. Use generic types in functions and structs
3. Explore trait bounds

**Exercises:**

1. Create a trait for shapes and implement it for different types
2. Write generic functions that work on multiple types
3. Use trait bounds to ensure type safety

**Weekend Project:**
Enhance the Turtle drawing simulator (part 4) - adding traits for different drawing capabilities.

## Week 12: Final Project

**Learning Goals:**

- Apply all concepts learned
- Design and implement a complete program
- Debug and refine code

**Activities:**

1. Plan and design a final project
2. Implement the project with guidance
3. Test and debug the project

**Project Ideas:**

1. A complete Turtle graphics application with:

   - Multiple turtle support
   - Saving and loading drawings
   - Custom commands and macros
   - A simple GUI (using a basic GUI library)

2. A text-based adventure game with:

   - Different rooms and items
   - Inventory management
   - Simple puzzles to solve
   - Saving and loading game state

3. A personal organiser with:
   - Task management
   - Calendar events
   - Notes and reminders
   - File storage for task attachments

**Final Celebration:**
Review all that your child has learned and celebrate their achievements with a small presentation of their final project!

## Beyond the 12 Weeks

**Continued Learning:**

- Explore modules and crates
- Learn about concurrency and threading
- Try building small web applications with Rust
- Contribute to open source Rust projects
- Join Rust communities (Reddit, Discord) for continued learning

**Recommended Resources:**

- The Rust Programming Language (book)
- Programming Rust (O'Reilly)
- Rust in Action (Manning)
- Rust Cookbook (Packt)
- Exercism.io Rust track
