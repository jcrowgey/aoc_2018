use std::collections::HashSet;
use std::io::BufRead;
use std::iter::FromIterator;

fn get_next_ready(sorted_tasks: &Vec<&u8>, deps: &HashSet<(u8, u8)>) -> Option<u8> {
    for t in sorted_tasks.iter() {
        // t is ready if it never appears on the right
        let mut ready = true;
        for (_before, then) in deps.iter() {
            if *t == then {
                ready = false;
                break;
            }
        }

        if ready {
            return Some(**t);
        }
    }
    None
}

pub fn seven_a<I>(buf: I) -> String
where
    I: BufRead,
{
    let mut all_tasks = HashSet::new();
    let mut deps = HashSet::new();
    for line in buf.lines() {
        let bytes: Vec<u8> = line.unwrap().bytes().collect();
        let before = bytes[5];
        let then = bytes[36];
        all_tasks.insert(before);
        all_tasks.insert(then);
        deps.insert((before, then));
    }

    let mut sorted_tasks = Vec::<_>::from_iter(all_tasks.iter());
    sorted_tasks.sort();

    let mut out = String::new();

    loop {
        let next_task = get_next_ready(&sorted_tasks, &deps).unwrap();
        out.push(next_task.clone() as char);

        // remove from the tasks remaining
        sorted_tasks.retain(|t| **t != next_task);

        deps = deps
            .iter()
            .cloned()
            .filter(|(before, _then)| next_task != *before)
            .collect();

        if sorted_tasks.len() == 0 {
            break;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &[u8; 49 * 7] = b"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";

    #[test]
    fn test_seven_a() {
        assert_eq!("CABDFE", seven_a(&INPUT[..]));
    }
}
