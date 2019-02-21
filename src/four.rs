use std::io::BufRead;
use std::io::ErrorKind::Other;
use std::collections::HashMap;
use std::iter::FilterMap;
use std::slice::Iter;

fn parse_guard(line: &String, start: usize) -> i32 {
    let start = start + 1;
    let mut end = start;
    for (i, c) in line[start + 1 ..].chars().enumerate() {
        if !c.is_digit(10) {
            end = start + i + 1;
            break;
        }
    }
    line[start..end].to_string().parse::<i32>()
                                .expect("problem parsing guard number")
}

fn parse_minute(line: &String) -> i32 {
    let left_bracket = line.find("]").unwrap();
    line[left_bracket-2 .. left_bracket].to_string()
                                        .parse::<i32>()
                                        .expect("problem parsing minute")
}

fn sort_bufread<I>(buf: I) -> Vec<String>
where
    I: BufRead,
{
    let mut lines_vec = Vec::new();
    for line in buf.lines() {
        lines_vec.push(line.unwrap());
    }
    lines_vec.sort_unstable();
    lines_vec
}

fn extract_guard_sleep<'a>(sorted_lines: &'a Vec<String>) -> FilterMap<Iter<'a, String>, Vec<String>>
{
    let mut cur_guard: i32 = -1; // XXX: this -1 business is nonsense
    let mut slept_from: i32 = -1; // XXX
    sorted_lines.iter().filter_map(|line| {
        if let Some(id_start) = line.find("#") {
            cur_guard = parse_guard(line, id_start);
            None
        }
        else if let Some(_) = line.find("falls asleep") {
            slept_from = parse_minute(line);
            None
        }
        else if let Some(_) = line.find("wakes up") {
            let awoke = parse_minute(line);
            let sleep_mins = Vec::<i32>::new();
            for i in slept_from..awoke {
                sleep_mins.push(i);
            }
            Some((cur_guard, sleep_mins))
        }
        else {
            panic!("we're screwed");
        }
    })
}

pub fn prob_4a<I>(buf: I) -> i32
where
    I: BufRead,
{

    let lines_vec = sort_bufread(buf);
    let mut sleep_counts = HashMap::<i32, Vec<i32>>::new();
    let mut cur_guard: i32 = -1; // XXX: this -1 business is nonsense
    let mut slept_from: i32 = -1; // XXX

    for line in lines_vec.iter() {
        if let Some(id_start) = line.find("#") {
            cur_guard = parse_guard(line, id_start);
        }
        else if let Some(_) = line.find("falls asleep") {
            slept_from = parse_minute(line);
        }
        else if let Some(_) = line.find("wakes up") {
            let awoke = parse_minute(line);
            let sleep_mins = sleep_counts.entry(cur_guard).or_insert(Vec::<i32>::new());
            for i in slept_from..awoke {
                sleep_mins.push(i);
            }
        }
        else {
            panic!("we're screwed");
        }
    }

    let mut sorted_guards: Vec<_> = sleep_counts.iter().collect();
    sorted_guards.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    let minutes = sorted_guards[0].1.to_owned();
    let mut minute_counts = HashMap::new();
    for m in minutes.iter() {
        *minute_counts.entry(m).or_insert(0) += 1;
    }
    let mut sorted_minutes: Vec<_> = minute_counts.iter().collect();
    sorted_minutes.sort_by(|a, b| b.1.cmp(a.1));

    *sorted_guards[0].0 * **sorted_minutes[0].0
}

/*
pub fn prob_4b<I>(buf: I) -> i32
where
    I: BufRead,
{
    unimiplemented!();
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &[u8; 569] = b"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn test_4a() {
        assert_eq!(240, prob_4a(&INPUT[..]));
    }

    #[test]
    fn test_4b() {
        assert_eq!(4455, prob_4a(&INPUT[..]));
    }

}
