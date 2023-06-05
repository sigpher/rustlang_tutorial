use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    guessing_game()
}

fn guessing_game() {
    println!("Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..100u8);
    loop {
        let mut guess = String::new();
        println!("Please input your guess: ");
        stdin().read_line(&mut guess).unwrap();

        let guess = guess.trim().parse::<u8>().unwrap();
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
