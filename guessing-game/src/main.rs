use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let rand_number: u32 = rand::thread_rng().gen_range(1..101);    

    loop {

        let mut guess = String::new();

        println!("Please input your guess (numbers only).\nIf you want to quit, type quit\n");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        if guess.to_lowercase().eq("quit\n") {
            println!("Thanks for playing!");
            break;
        }

        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess);

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            },
        }
    }
}
