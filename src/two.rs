use std::io::{self, BufRead};
use std::collections::HashMap;

pub fn prob_2a() {
    let stdin = io::stdin();
    let mut two_count = 0;
    let mut three_count = 0;
    for line in stdin.lock().lines() {
        let mut char_counts = HashMap::new();
        for chr in line.unwrap().chars() {
            match char_counts.get(&chr) {
                Some(count) => char_counts.insert(chr, count + 1),
                None => char_counts.insert(chr, 1),
            };
        }


        let mut two_flag = false;
        let mut three_flag = false;
        for val in char_counts.values() {
            if !two_flag {
                if val == &2 {
                    two_count += 1;
                    two_flag = true;
                }
            }
            if !three_flag {
                if val == &3 {
                    three_count += 1;
                    three_flag = true;
                }
            }

            if two_flag && three_flag {
                break;
            }
        }
    }
    println!("{}", two_count * three_count);
}
