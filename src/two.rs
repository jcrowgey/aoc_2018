use std::collections::HashMap;
use std::io::BufRead;

pub fn two_a<I>(buf: I) -> i32
where
    I: BufRead,
{
    let mut two_count = 0;
    let mut three_count = 0;
    for line in buf.lines() {
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
    two_count * three_count
}

pub fn two_b<I>(buf: I) -> String
where
    I: BufRead,
{
    let mut ids = Vec::<String>::new();
    let mut answer = String::from("i give up");
    'outer: for x in buf.lines() {
        let x = x.unwrap();
        for y in ids.iter() {
            let sames: Vec<(char, char)> =
                x.chars().zip(y.chars()).filter(|p| p.0 == p.1).collect();
            if sames.len() == (x.len() - 1) {
                answer = sames.iter().map(|x| x.0).collect();
                break 'outer;
            }
        }
        ids.push(x.clone());
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_a() {
        let input = b"abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n";
        assert_eq!(12, two_a(&input[..]));
    }

    #[test]
    fn test_two_b() {
        let input = b"abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n";
        assert_eq!("fgij".to_string(), two_b(&input[..]));
    }
}
