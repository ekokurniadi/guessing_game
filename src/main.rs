use rand::Rng;
// trait
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Menampung random number
    // Dengan range 1 s/d 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        // tangkap input dari user
        io::stdin()
            .read_line(&mut guess) // return enum Result
            .expect("Failed to read line");

        // shadowing variabel guess ke int
        // checking input is only number
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // compare guess with Ordering
        match guess.cmp(&secret_number) {
            // arm
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
