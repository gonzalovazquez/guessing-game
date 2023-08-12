use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // Created a mutable variable that is bounded to a new
    // empty instance of String
    let mut guess = String::new();

    io::stdin()
        // The & indicates that this argument is a reference,
        // which gives you a way to let multiple parts of your
        // code access one piece of data without needing to copy
        // that data into memory multiple times.
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
