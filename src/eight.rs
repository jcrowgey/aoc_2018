use std::io::BufRead;

fn node_value(mut input: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    match input.pop() {
        Some(i) => {
            let mut node_children: i32 = i;
            let mut meta_cnt: i32 = input.pop().unwrap();
            let mut child_values = vec![0i32]; // problem requires 1-based indexing

            if node_children > 0 {
                while node_children > 0 {
                    child_values.push(node_value(&mut input));
                    node_children -= 1;
                }

                while meta_cnt > 0 {
                    let child: usize = input.pop().unwrap() as usize;
                    if child < child_values.len() {
                        sum += child_values[child];
                    }
                    meta_cnt -= 1;
                }
            } else {
                while meta_cnt > 0 {
                    let to_add: i32 = input.pop().unwrap();
                    sum += to_add;
                    meta_cnt -= 1;
                }
            }
        }
        None => (),
    };
    sum
}

fn sum_metas(mut input: &mut Vec<i32>) -> i32 {
    let mut sum = 0;
    match input.pop() {
        Some(i) => {
            let mut node_children: i32 = i;
            let mut meta_cnt: i32 = input.pop().unwrap();

            while node_children > 0 {
                sum += sum_metas(&mut input);
                node_children -= 1;
            }

            while meta_cnt > 0 {
                let to_add: i32 = input.pop().unwrap();
                sum += to_add;
                meta_cnt -= 1;
            }
        }
        None => (),
    };
    sum
}

fn prepare_input<I>(mut buf: I) -> Vec<i32>
where
    I: BufRead,
{
    let mut input = Vec::<u8>::new();

    let _read = buf.read_until(10u8, &mut input);
    let input = String::from_utf8(input).unwrap();
    let input: Vec<&str> = input.split_whitespace().collect();
    let mut input: Vec<i32> = input.iter().map(|s| s.parse().unwrap()).collect();
    input.reverse();
    input
}

pub fn eight_a<I>(buf: I) -> i32
where
    I: BufRead,
{
    let mut input = prepare_input(buf);
    sum_metas(&mut input)
}

pub fn eight_b<I>(buf: I) -> i32
where
    I: BufRead,
{
    let mut input = prepare_input(buf);
    node_value(&mut input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &[u8; 36] = b"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n";

    #[test]
    fn test_eight_a() {
        assert_eq!(138, eight_a(&INPUT[..]));
    }

    #[test]
    fn test_eight_b() {
        assert_eq!(66, eight_b(&INPUT[..]));
    }
}
