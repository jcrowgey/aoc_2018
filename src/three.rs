use std::collections::HashMap;
use std::io::BufRead;

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
    dirty
        .trim_start_matches(left)
        .trim_start()
        .trim_end()
        .to_string()
        .parse::<i32>()
        .expect("problem parsing number")
}

pub fn three_a<I>(buf: I) -> i32
where
    I: BufRead,
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
    let two_or_more: Vec<&i32> = use_counts.values().filter(|c| c >= &&2).collect();
    two_or_more.len() as i32
}

pub fn three_b<I>(buf: I) -> i32
where
    I: BufRead,
{
    let mut overlaps = HashMap::<i32, Vec<i32>>::new();
    let mut square_use = HashMap::<String, Vec<i32>>::new();
    for line in buf.lines() {
        let line = line.unwrap();
        let c = Claim::from_line(line);

        let mut this_claim_overlaps = Vec::<i32>::new();

        for x in c.start_x..c.start_x + c.claim_x {
            for y in c.start_y..c.start_y + c.claim_y {
                let square_key = format!("{},{}", x, y);
                let users = square_use.entry(square_key).or_insert(Vec::<i32>::new());

                for user in users.iter() {
                    this_claim_overlaps.push(*user);
                    let user_overlaps = overlaps.entry(*user).or_insert(Vec::<i32>::new());
                    user_overlaps.push(c.id);
                }

                users.push(c.id);
            }
        }

        overlaps.insert(c.id, this_claim_overlaps);
    }

    let mut answer: i32 = -1;
    for (k, v) in overlaps.iter() {
        if v.len() == 0 {
            answer = *k;
            break;
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &[u8; 42] = b"#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2\n";

    #[test]
    fn test_three_a() {
        assert_eq!(4, three_a(&INPUT[..]));
    }

    #[test]
    fn test_three_b() {
        assert_eq!(3, three_b(&INPUT[..]));
    }
}
