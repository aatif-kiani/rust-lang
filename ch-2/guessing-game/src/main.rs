use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand_guess = rand::thread_rng().gen_range(1, 11);
    println!("Guess the number!");
    println!("Input the guess number ");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Input failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid value, Enter again");
                continue;
            }
        };

        match guess.cmp(&rand_guess) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
