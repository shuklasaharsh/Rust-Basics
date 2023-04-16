mod guess;

use guess::guess_game;

fn main() {
    loop {
        match guess_game() {
            0 => {
                println!("You win :)");
                break;
            },
            1 => println!("Too Big"),
            -1 => println!("Too Small"),
            _ => {
                println!("What just happened?");
            }
        }
    }

}