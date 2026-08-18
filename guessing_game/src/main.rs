use std::{cmp::Ordering, io};

fn main() {
    let mut guest_count: u16 = 0;
    let secret_number = rand::random_range(1..=100);

    println!("Guess the number!");

    println!("Please input your guess 1 to 100.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error parse number {guess}");
                continue;
            }
        };

        println!("You guessed: {guess}");

        guest_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!, guest {guest_count} times.");
                break;
            }
        }
    }
}
