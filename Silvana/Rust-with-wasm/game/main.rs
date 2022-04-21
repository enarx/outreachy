extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guess the number!!");

    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("type your guess here");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("failed to read input");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you said the number is: {}", palpite);

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Too low, try again"),
            Ordering::Greater => println!("Quiet, too loud, try again"),
            Ordering::Equal => {
                println!("You got it right, congratulations!");
                break;
            }
        }
    }
}

