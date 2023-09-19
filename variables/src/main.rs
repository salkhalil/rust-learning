mod games;
mod util;
mod xmas;

use games::{celcius_to_far, fibonacci};
use util::take_number;
use xmas::twelve_days;

fn main() {
    let select_msg = "Please pick which program you want to run: \n1) Celcius to Fahrenheit \n2) Fibonacci \n3) Twelve Days of Christmas";

    let guess = take_number(select_msg.to_string());

    match guess {
        1 => celcius_to_far(),
        2 => fibonacci(),
        3 => twelve_days(),
        _ => {
            println!("Please pick a valid number");
            return;
        }
    }
}
