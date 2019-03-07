use std::collections::HashMap;
use std::io::BufRead;

fn prob1_adder(mut freq: i32, mut line: String) -> i32 {
    let sign_str: String = line.drain(..1).collect();
    let mut sign = 1;
    if sign_str == "-" {
        sign = -1;
    }
    let i = line.parse::<i32>().expect("problem parsing number");
    freq += i * sign;
    freq
}

pub fn one_a<I>(buf: I) -> i32
where
    I: BufRead,
{
    let mut freq: i32 = 0;
    for line in buf.lines() {
        let line = line.unwrap();
        freq = prob1_adder(freq, line);
    }
    freq
}

pub fn one_b<I>(buf: I) -> i32
where
    I: BufRead,
{
    let mut seen_freqs = HashMap::new();
    let mut freq: i32 = 0;
    seen_freqs.insert(freq, true);

    // save lines for rerun
    let mut replay_lines = Vec::new();

    for line in buf.lines() {
        let line = line.unwrap();
        replay_lines.push(line.clone());
        freq = prob1_adder(freq, line);

        if seen_freqs.contains_key(&freq) {
            return freq;
        }
        seen_freqs.insert(freq, true);
    }

    loop {
        for line in replay_lines.iter() {
            freq = prob1_adder(freq, line.to_string());
            if seen_freqs.contains_key(&freq) {
                return freq;
            }
            seen_freqs.insert(freq, true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_a() {
        let input = b"+1\n-2\n+3\n+1\n";
        assert_eq!(3, one_a(&input[..]));
    }

    #[test]
    fn test_one_b() {
        let input = b"+1\n-2\n+3\n+1\n";
        assert_eq!(2, one_b(&input[..]));

        let input = b"+1\n-1\n";
        assert_eq!(0, one_b(&input[..]));

        let input = b"+3\n+3\n+4\n-2\n-4\n";
        assert_eq!(10, one_b(&input[..]));

        let input = b"-6\n+3\n+8\n+5\n-6\n";
        assert_eq!(5, one_b(&input[..]));

        let input = b"+7\n+7\n-2\n-7\n-4\n";
        assert_eq!(14, one_b(&input[..]));
    }
}
