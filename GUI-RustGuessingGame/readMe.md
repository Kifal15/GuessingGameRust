
# Guess the Number Game
This project is a simple implementation of the "Guess the Number" game using Rust and the FLTK library. The user is prompted to guess a number between `1-100`, and the program will indicate if the guess is `too small`, `too big`, or `correct`. If the guess is correct, the user is prompted to guess a new number.

## Code Explanation
`use fltk::{app::*, button::*, frame::*, input::*, window::*};`
`use fltk::prelude::WidgetBase;`
`use fltk::prelude::WidgetExt;`
`use fltk::prelude::GroupExt;`
`use fltk::prelude::InputExt;`
`use fltk::enums::Color;`
`use std::cmp::Ordering;`
`use rand::Rng;`
The first part of the code imports the necessary FLTK and Rust libraries for the program to run.

`fn randomnumber() -> i32 {`
    `let mut rng = rand::thread_rng();`
    `let secret_number = rng.gen_range(1..=100);`
    `return secret_number;`
`}`
This function generates a random number between `1` and `100` using the `rand` crate.

`fn main() {`
    `let app = App::default();`
    `let mut wind = Window::new(100, 100, 400, 300, "Guess the number");`
    `let mut frame = Frame::new(20, 20, 300, 40, "Guess a number between 1-100");`
    `let mut input = Input::new(100,60,200,30,"");`
    `let mut button = Button::new(50, 100, 150, 40, "Guess");`
    `input.set_text_size(20);`
    `input.set_value("");`
    `wind.end();`
    `wind.show();`
    `wind.set_color(Color::Yellow);`
    `frame.set_color(Color::Red);`
This section creates the window, frames, input field, and button for the program. It also sets the color for the window and frames.

    // Set callback for the guess button
    `let mut secret_number = randomnumber();`
    `button.set_callback(move|c| {`
       `let guess: i32 = match input.value().trim().parse() {`
            `Ok(num) => num,`
            `Err(_) => return,`
        `};`
        `let result_msg = match guess.cmp(&secret_number) {`
            `Ordering::Less => "Too small !",`
            `Ordering::Greater => "Too big !",`
            `Ordering::Equal =>  {`
                `secret_number = randomnumber();`
                `input.set_value("");`
                `"You win! Guess a number between 1-100"`
            `},`
        `};`
        `frame.set_label(result_msg);`
        
    `});`
This section sets the callback for the guess button. When the button is pressed, the program will check if the user's guess is `too small`, `too big`, or `correct`. If the guess is correct, the program will generate a new random number and prompt the user to guess again.

    `app.run().unwrap();`
`}`
This section runs the FLTK application.

## Installation
To run this project, you will need to have Rust and FLTK installed on your computer. You can download and install Rust from the following link:

[Rust Installation](https://www.rust-lang.org/learn/get-started)

You can install FLTK by adding the following lines to your `Cargo.toml` file:

```
[dependencies]
fltk = "1.3"
rand = "0.7"
```

## Usage
Clone the project repository to your local machine.

`git clone https://github.com/Kifal15/GuessingGameRust/new/main/GUI-RustGuessingGame`

Navigate to the project directory.

`cd GUI-RustGuessingGame`

Run the project.

`cargo run`

The program will launch a window prompting the user to guess a number between `1-100`. Enter a number in the input field and click the "Guess" button.
The program will indicate if the guess is `too small`, `too big`, or `correct`. If the guess is correct, the user is prompted to guess a new number.

## License
This project is licensed under the MIT License.
