# Guess the Number Game 
This project is a simple implementation of the "Guess the Number" game using Rust and the [Rustyline library](https://github.com/rust-lang/rustyline). The user is prompted to guess a number between 1-100, and the program will indicate if the guess is too small, too big, or correct.

## Code Explanation
```
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::cmp::Ordering;
use rand::Rng;
```
The first part of the code imports the necessary Rust libraries for the program to run.

```
fn main() {
    println!("Guess the number!");
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);
```
This section generates a random number between 1 and 100 using the `rand` crate and prompts the user to guess the number.

```
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("Please input your guess the number is between 1-100: ");
        match readline {
            Ok(line) => {
                let guess: i32 = match line.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                println!("You guessed: {}", guess);

                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        return;
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
```
This section sets up a loop that prompts the user to guess the number and checks if the guess is too small, too big, or correct. The program will continue to prompt the user until the correct number is guessed or the user interrupts the loop.

## Installation
To run this project, you will need to have Rust and the `Rustyline` crate installed on your computer. You can download and install Rust from the following [link](https://www.rust-lang.org/tools/install): 

You can install Rustyline by adding the following line to your `Cargo.toml` file:

```
[dependencies]
rustyline = "8.5"
rand = "0.7"
```

## Usage
Clone the project repository to your local machine.

```
git clone https://github.com/Kifal15/GuessingGameRust/new/main/CLI-RustBasedGuessingGame
```

Navigate to the project directory.

```
cd CLI-RustBasedGuessingGame
```

Run the project.

```
cargo run
```

The program will prompt the user to guess a number between 1-100. Enter a number and press enter.
The program will indicate if the guess is too small, too big, or correct. If the guess is correct, the user wins the game.

## License
This project is licensed under the MIT License.
