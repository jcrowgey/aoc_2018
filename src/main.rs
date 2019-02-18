use std::env::args;
use std::io::{self};

mod one;
mod two;
mod three;


fn main() {
    let args: Vec<_> = args().collect();
    if args.len() > 1 {
        let stdin = io::stdin();
        let buf = stdin.lock();
        match args[1].as_ref() {
            "1a" => println!("{}", one::prob_1a(buf)),
            "1b" => println!("{}", one::prob_1b(buf)),
            "2a" => println!("{}", two::prob_2a(buf)),
            "2b" => println!("{}", two::prob_2b(buf)),
            "3a" => println!("{}", three::prob_3a(buf)),
            "3b" => println!("{}", three::prob_3b(buf)),
            _ => println!("idk that"),
        }
    }
}
