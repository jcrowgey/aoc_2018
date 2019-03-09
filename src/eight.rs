use std::io::BufRead;

fn sum_metas(mut input: &mut Vec<&str>) -> i32 {
    let mut sum = 0;
    match input.pop() {
        Some(i) => {
            let mut node_children: i32 = i.parse().unwrap();
            let mut meta_cnt: i32 = input.pop().unwrap().parse().unwrap();

            while node_children > 0 {
                sum += sum_metas(&mut input);
                node_children -= 1;
            }

            while meta_cnt > 0 {
                let to_add: i32 = (*input).pop().unwrap().parse().unwrap();
                sum += to_add;
                meta_cnt -= 1;
            }
        }
        None => (),
    };
    sum
}

pub fn eight_a<I>(mut buf: I) -> i32
where
    I: BufRead,
{
    let mut input = Vec::<u8>::new();

    let _res = buf.read_until(10u8, &mut input);
    let input = String::from_utf8(input).unwrap();
    let mut input: Vec<&str> = input.split_whitespace().collect();
    input.reverse();

    sum_metas(&mut input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &[u8; 36] = b"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2\n";

    #[test]
    fn test_eight_a() {
        assert_eq!(138, eight_a(&INPUT[..]));
    }
}
