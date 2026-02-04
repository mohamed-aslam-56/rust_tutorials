use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("**********Guessing Game**********");

    let target=rand::thread_rng().gen_range(1..=10);
    loop{
        let mut guess=String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Number not entered!");

        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };

        println!("You guessed : {guess}");

        match guess.cmp(&target){
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{
                println!("You win!");
                break;
            }
        }


    }
}

