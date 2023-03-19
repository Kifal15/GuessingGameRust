use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);

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
}
