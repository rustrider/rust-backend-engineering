use std::cmp::Ordering;
use std::io;
use rand::Rng;

// comment
fn main() {
    println!("guess the number!");

    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);
    io::stdin().
        read_line(&mut guess)
        .expect("something went wrong");
    let guess: u32 = guess.trim().parse().expect("please type a number");

    loop {
        println!("“Please input your guess!");
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => {
                println!("try with less");
                continue;
            }
            Ordering::Less => {
                println!("try with more");
                continue;
            }
        }

        println!("you guessed: {guess}");
    }
    //println!("the secret number is: {secret_number}");

}