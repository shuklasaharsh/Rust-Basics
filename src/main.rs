use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;


fn main() {
    let mut rng = thread_rng();
    println!("guess the number!");

    // We ask the user to input the number
    println!("Enter the number: ");

    // Create a variable
    let mut guess = String::new();

    // Input the variable
    io::stdin().read_line(&mut guess).expect("Cannot read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");


    let secret_number: u32 = rng.gen_range(1..10);

    // Now lets compare the guess with the secret number
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("You win! :)"),
    }
}
