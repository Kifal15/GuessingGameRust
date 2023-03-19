use fltk::{app::*, button::*, frame::*, input::*, window::*};
use fltk::prelude::WidgetBase;
use fltk::prelude::WidgetExt;
use fltk::prelude::GroupExt;
use fltk::prelude::InputExt;
use fltk::enums::Color;
use std::cmp::Ordering;
use rand::Rng;
fn randomnumber() -> i32 {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);
    return secret_number;
}
fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Guess the number");
    let mut frame = Frame::new(20, 20, 300, 40, "Guess a number between 1-100");
    let mut input = Input::new(100,60,200,30,"");
    let mut button = Button::new(50, 100, 150, 40, "Guess");
    input.set_text_size(20);
    input.set_value("");
    wind.end();
    wind.show();
    wind.set_color(Color::Yellow);
    frame.set_color(Color::Red);
    // Set callback for the guess button
    let mut secret_number = randomnumber();
    button.set_callback(move|c| {
       let guess: i32 = match input.value().trim().parse() {
            Ok(num) => num,
            Err(_) => return,
        };
        let result_msg = match guess.cmp(&secret_number) {
            Ordering::Less => "Too small! Guess something smaller than this",
            Ordering::Greater => "Too big! Guess something smaller than this",
            Ordering::Equal =>  {
                secret_number = randomnumber();
                input.set_value("");
                "You win! Guess a number between 1-100"
            },
        };
        frame.set_label(result_msg);
        
    });
    app.run().unwrap();
}