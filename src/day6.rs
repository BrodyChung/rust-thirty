// ref: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_number() {
    println!("Guess the number!");

    // or gen_range(1, 101)
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut score: i32 = 0; // thre is no int

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        score += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Your score is {score}.");
                break;
            }
        }
    }
}