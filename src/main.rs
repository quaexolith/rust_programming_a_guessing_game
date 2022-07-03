use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // immutable by default; mut makes it mutable

    // read_line returns a Result enum
    // Possible values: Ok, Err (referred to as variants)
    // This encodes the value in Ok
    // or what went wrong in Err
    io::stdin()
        .read_line(&mut guess) // & indicates passing by reference
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
