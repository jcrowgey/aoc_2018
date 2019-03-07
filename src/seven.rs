use std::collections::{HashMap, HashSet};
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

fn parse_input<I>(buf: I) -> (HashSet<u8>, HashSet<(u8, u8)>)
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
    (all_tasks, deps)
}

pub fn seven_a<I>(buf: I) -> String
where
    I: BufRead,
{
    let (all_tasks, mut deps) = parse_input(buf);
    let mut sorted_tasks = Vec::<_>::from_iter(all_tasks.iter());
    sorted_tasks.sort();

    let mut out = String::new();

    loop {
        let next_task = get_next_ready(&sorted_tasks, &deps).unwrap();
        out.push(next_task.clone() as char);

        // remove from the tasks remaining
        sorted_tasks.retain(|t| **t != next_task);
        deps.retain(|(before, _then)| next_task != *before);

        if sorted_tasks.len() == 0 {
            break;
        }
    }

    out
}

pub fn seven_b<I>(buf: I, worker_cnt: i32, task_min: i32) -> i32
where
    I: BufRead,
{
    let (all_tasks, mut deps) = parse_input(buf);
    let mut sorted_tasks = Vec::<_>::from_iter(all_tasks.iter());
    sorted_tasks.sort();

    let mut ticks: i32 = 0;
    let mut task_clock = HashMap::new();
    loop {
        for clock in task_clock.values_mut() {
            *clock -= 1;
        }
        for (task, clock) in task_clock.iter() {
            if *clock == 0 {
                deps.retain(|(before, _then)| *task != *before);
            }
        }
        task_clock.retain(|_, v| *v != 0);

        while task_clock.len() < worker_cnt as usize {
            match get_next_ready(&sorted_tasks, &deps) {
                Some(next_task) => {
                    task_clock.insert(next_task, task_min + next_task as i32 - 64);
                    sorted_tasks.retain(|t| **t != next_task);
                }
                None => {
                    break;
                }
            };
        }

        if task_clock.len() == 0 {
            break;
        }

        ticks += 1;
    }
    ticks
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

    #[test]
    fn test_seven_b() {
        let worker_cnt = 2;
        let task_min = 0;
        assert_eq!(15, seven_b(&INPUT[..], worker_cnt, task_min));
    }
}
