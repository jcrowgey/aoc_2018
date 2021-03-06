use std::env::args;
use std::io;

mod one;
mod two;
mod three;
mod four;
mod five;
mod six;
mod seven;
mod eight;
mod nine;
mod ten;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() > 1 {
        let stdin = io::stdin();
        let buf = stdin.lock();
        match args[1].as_ref() {
            "1a" => println!("{}", one::one_a(buf)),
            "1b" => println!("{}", one::one_b(buf)),
            "2a" => println!("{}", two::two_a(buf)),
            "2b" => println!("{}", two::two_b(buf)),
            "3a" => println!("{}", three::three_a(buf)),
            "3b" => println!("{}", three::three_b(buf)),
            "4a" => println!("{}", four::four_a(buf)),
            "4b" => println!("{}", four::four_b(buf)),
            "5a" => println!("{}", five::five_a(buf)),
            "5b" => println!("{}", five::five_b(buf)),
            "6a" => println!("{}", six::six_a(buf)),
            "6b" => println!("{}", six::six_b(buf, 10000)),
            "7a" => println!("{}", seven::seven_a(buf)),
            "7b" => println!("{}", seven::seven_b(buf, 5, 60)),
            "8a" => println!("{}", eight::eight_a(buf)),
            "8b" => println!("{}", eight::eight_b(buf)),
            "9a" => println!("{}", nine::nine_a(buf)),
            "9b" => println!("{}", nine::nine_b(buf)),
            "10" => ten::ten(buf),
            _ => println!("idk that"),
        }
    }
}
