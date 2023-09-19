pub fn take_number(msg: String) -> usize {
    let mut guess = String::new();

    println!("{}", msg);
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: usize = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("\nPlease enter a valid number!\n");
            return take_number(msg);
        }
    };
    guess
}
