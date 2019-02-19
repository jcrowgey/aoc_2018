use std::io::BufRead;

pub fn prob_4a<I>(buf: I) -> i32
where
    I: BufRead,
{
    unimplemented!();
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

    #[test]
    fn test_4a() {
        let input = b"[1518-11-01 00:00] Guard #10 begins shift
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
        assert_eq!(240, prob_4a(&input[..]));
    }
}
