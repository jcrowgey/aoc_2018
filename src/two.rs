use std::io::BufRead;
use std::collections::HashMap;

pub fn prob_2a<I>(buf: I) -> i32
where
    I: BufRead
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

pub fn prob_2b<I>(buf: I) -> String
where
    I: BufRead
{
    "".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2a() {
        let input = b"abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab\n";
        assert_eq!(12, prob_2a(&input[..]));
    }

    #[test]
    fn test_2b() {
        let input = b"abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz\n";
        assert_eq!("fgij".to_string(), prob_2b(&input[..]));
    }
}
