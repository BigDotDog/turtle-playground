# From Logo to Rust: A Child-Friendly Guide

## Welcome to Rust!

Hi there! You already know how to program in Logo, which is brilliant! Now you're going to learn Rust, a powerful language used by professional programmers to build everything from games to websites.

## What Makes Rust Special?

- **Safety**: Rust helps you avoid common programming mistakes.
- **Speed**: Rust programs run super fast.
- **Fun**: You can build really cool things with Rust!

## Getting Started: Your First Rust Program

Let's write your first Rust program! Open your terminal and create a new project:

```shell
cargo new hello_rust
cd hello_rust
```

Open the `src/main.rs` file in your code editor. You'll see this code:

```rust
fn main() {
    println!("Hello, world!");
}
```

Let's understand what this does:

- `fn main()` creates a function called "main" - this is where your program starts.
- `println!("Hello, world!");` prints text to the screen.
- The `!` means this is a "macro" - a special kind of function in Rust.

Run your program by typing `cargo run` in the terminal!

## From Logo to Rust: Comparing What You Know

### Variables

In Logo, you might have used:

```text
make "name "Claude
print :name
```

In Rust, you'll write:

```rust
let name = "Claude";
println!("{name}");
```

### Loops

In Logo, you might have used:

```text
repeat 4 [fd 100 rt 90]
```

In Rust, you can write:

```rust
for _ in 0..4 {
    // Code to repeat 4 times
}
```

### Conditionals

In Logo, you might have written:

```text
if :x > 10 [print "big] [print "small]
```

In Rust, this looks like:

```rust
if x > 10 {
    println!("big");
} else {
    println!("small");
}
```

## Fun First Projects to Try

1. **Number Guessing Game**: Create a game where the computer picks a random number and you have to guess it!
2. **Maths Quiz**: Make a program that asks maths questions and keeps score of correct answers.
3. **Text Adventure**: Create a simple adventure game where you make choices by typing commands.

## Remember

- Don't worry if you make mistakes - professional programmers make mistakes all the time!
- Rust will help you find and fix your mistakes.
- The compiler is your friend, even when it seems strict.
- Have fun and experiment!

Next time, we'll learn about different types of data in Rust!
