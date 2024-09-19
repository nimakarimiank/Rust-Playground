use std::cmp::Ordering;
use std::io;
use rand::Rng;

//Chapter 2
pub fn guessing_game() {
    //Chapter 2
    loop {
        println!("Enter a random number please:\t");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("*Failed to read line\t");
        let guess = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("The entered value is not a sole number.");
                continue;
            }
        };
        println!("You guessed {guess}");
        let random_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is: {random_number}");
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    //
}
//