extern crate rand;

use rand::Rng;
use std::env;
use std::str::FromStr;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <highest_page_number>", args[0]);
        println!("Prints a random page number from a book with specified number of pages");
        process::exit(1);
    }
    let highest_page = usize::from_str(&args[1]).unwrap();
    let mut rng = rand::thread_rng();
    println!("{}", rng.gen_range(1, highest_page + 1));
}
