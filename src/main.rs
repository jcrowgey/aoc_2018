use std::env::args;

mod one;
mod two;


fn main() {
    let args: Vec<_> = args().collect();
    if args.len() > 1 {
        match args[1].as_ref() {
            "1a" => one::prob_1a(),
            "1b" => one::prob_1b(),
            "2a" => two::prob_2a(),
            _ => println!("idk that"),
        }
    }
}
