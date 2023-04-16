mod guess;
mod data_types;

use std::thread::sleep;
use std::time::Duration;


fn main() {
    let dur = Duration::new(1, 0);
    let mut count: u8 = 10;
    while count != 0 {
        println!("{count}");
        count -= 1;
        sleep(dur);
    }
    println!("LIFT OFF!")
}