use std::io::BufRead;
use std::collections::HashMap;

#[derive(Debug)]
struct Claim {
    id: i32,
    start_x: i32,
    start_y: i32,
    claim_x: i32,
    claim_y: i32,
}

impl Claim {
    fn from_line(mut line: String) -> Claim {
        // trim leading #
        let at_off = line.find("@").unwrap();
        let id: String = line.drain(..at_off).collect();
        let id = clean_and_parse(id, "#");

        let colon_off = line.find(":").unwrap();
        let mut start: String = line.drain(..colon_off).collect();

        let comma_off = start.find(",").unwrap();
        let start_x: String = start.drain(..comma_off).collect();
        let start_x = clean_and_parse(start_x, "@");

        let start_y: String = start;
        let start_y = clean_and_parse(start_y, ",");

        let x_off = line.find("x").unwrap();
        let claim_x: String = line.drain(..x_off).collect();
        let claim_x = clean_and_parse(claim_x, ":");

        let claim_y: String = line;
        let claim_y = clean_and_parse(claim_y, "x");

        Claim {
            id: id,
            start_x: start_x,
            start_y: start_y,
            claim_x: claim_x,
            claim_y: claim_y,
        }
    }
}

fn clean_and_parse(dirty: String, left: &str) -> i32 {
    dirty.trim_start_matches(left)
         .trim_start()
         .trim_end()
         .to_string()
         .parse::<i32>().expect("problem parsing number")
}

pub fn prob_3a<I>(buf: I) -> i32
where
    I: BufRead
{

    let mut use_counts = HashMap::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let c = Claim::from_line(line);
        for x in c.start_x..c.start_x + c.claim_x {
            for y in c.start_y..c.start_y + c.claim_y {
                let key = format!("{},{}", x, y);
                match use_counts.get(&key) {
                    Some(count) => use_counts.insert(key, count + 1),
                    None => use_counts.insert(key, 1),
                };
            }
        }
    }
    let mut two_or_more = 0;
    for count in use_counts.values() {
        if count >= &2 {
            two_or_more += 1;
        }
    }

    two_or_more
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3a() {
        let input = b"#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";
        assert_eq!(4, prob_3a(&input[..]));
    }
}
