// chapter 2

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut attempts = 0;
    let max_attempts = 10;

    loop {
        if attempts >= max_attempts {
            println!("Game over ! The number was {}", secret_number);
            break;
        }

        println!("Please enter a number ! (attempt {}/{})", attempts + 1, max_attempts);

        let mut supposition = String::new();

        io::stdin()
            .read_line(&mut supposition)
            .expect("Failed reading user's entry");

        let supposition: u32 = match supposition.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
            
        };

        attempts += 1;

        match supposition.cmp(&secret_number) {
            Ordering::Less => println!("More !"),
            Ordering::Greater => println!("Less !"),
            Ordering::Equal => {
                println!("You win in {} attempts !", attempts);
                break;
            }
        }
    }
}
