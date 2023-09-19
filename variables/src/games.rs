use crate::take_number;

pub fn celcius_to_far() {
    let cel = take_number("Please enter your value in Degrees celcius:".to_string());
    let cel: f32 = cel as f32;

    let far = (cel * 1.8) + 32 as f32;

    println!("Celcius: {cel}˚C \nFarenheit: {far:.2}˚F");
}

pub fn fibonacci() {
    let n = take_number(
        "Please enter the number of fibonacci numbers you want to generate".to_string(),
    );

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut numbers: Vec<u128> = Vec::new();

    for _ in 1..(n + 1) {
        let c = match a.checked_add(b) {
            Some(c) => c,
            None => {
                println!("Sorry, can't add {} and {}", a, b);
                0
            }
        };
        a = b;
        b = c;
        numbers.push(a);
    }
    println!("First {n} Fibonacci numbers: {:?}", numbers);
    println!(
        "{n}th Fibonacci number: {}",
        numbers.last().expect("Sorry, can't get last")
    );
    println!("Length of sequence: {}", numbers.len());
}
