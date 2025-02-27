use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Please guess number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Input your guess!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("!Failed to input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to convert!");
                continue;
            },
        };
        println!("Your guess is {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), 
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
