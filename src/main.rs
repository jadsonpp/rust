use std::io;
use rand::Rng;
use std::cmp::Ordering;
/*
    Book from Rust.
    https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
*/

fn main() {
    //Generate a new number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop{
        //mut make a variable mutable, by default they aren't mutable.
        let mut guess = String::new();
        println!("Please input your guess.");
        //Read input line
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        /*  
            Parse a String to number;
            trim() eliminated any whitespace
            u32 contain only numerical characters
            parse method used to parse string into some kind of number
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp (&secret_number){
            Ordering::Less=> println!("Smaller!"),
            Ordering::Greater=>println!("Bigger!"),
            Ordering::Equal=> {
                println!("Win!");
                break;
            }
        }

    }
    
}
