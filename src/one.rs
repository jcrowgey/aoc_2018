use std::io::{self, BufRead};
use std::collections::HashMap;


fn prob1_adder(mut freq: i64, mut line: String) -> i64 {
    let sign_str: String = line.drain(..1).collect();
    let mut sign = 1;
    if sign_str == "-" {
        sign = -1;
    }
    let i = line.parse::<i64>().expect("problem parsing number");
    freq += i * sign;
    freq
}


pub fn prob_1a() {
    let stdin = io::stdin();
    let mut freq: i64 = 0;
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        freq = prob1_adder(freq, line);
    }
    println!("{}", freq);
}


pub fn prob_1b() {
    let stdin = io::stdin();
    let mut seen_freqs = HashMap::new();
    let mut freq: i64 = 0;
    seen_freqs.insert(freq, true);

    // save lines for rerun
    let mut replay_lines = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        replay_lines.push(line.clone());
        freq = prob1_adder(freq, line);

        if seen_freqs.contains_key(&freq) {
            println!("{}", freq);
            break
        }
        seen_freqs.insert(freq, true);
    }

    'outer: loop {
        for line in replay_lines.iter() {
            freq = prob1_adder(freq, line.to_string());
            if seen_freqs.contains_key(&freq) {
                println!("{}", freq);
                break 'outer;
            }
            seen_freqs.insert(freq, true);
        }
    }
}
