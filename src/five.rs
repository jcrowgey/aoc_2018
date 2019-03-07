use std::io::BufRead;
use std::u32;

fn char_eq(a: i8, b: i8) -> bool {
    (a - b).abs() == 32
}

fn react_polymer(inp: &Vec<u8>) -> Vec<i8> {
    let mut out = Vec::<i8>::new();
    for ubyte in inp.iter() {
        let byte: i8 = *ubyte as i8;
        let out_len = out.len();
        if out_len > 0 && char_eq(out[out_len - 1], byte) {
            out.pop();
        } else {
            out.push(byte);
        }
    }
    out
}

pub fn five_b<I>(mut buf: I) -> usize
where
    I: BufRead,
{
    let mut inp = Vec::<u8>::new();
    let _res = buf.read_until(10u8, &mut inp);
    assert_eq!(inp.pop(), Some(10u8));

    let mut shortest = u32::MAX as usize;
    for val in 65i8..90i8 {
        // A .. Z
        let f = inp
            .iter()
            .filter(|&&b| !(char_eq(val, b as i8) || val == (b as i8)))
            .map(|&b| b)
            .collect();
        let reacted = react_polymer(&f);
        if reacted.len() < shortest {
            shortest = reacted.len()
        }
    }
    shortest
}

pub fn five_a<I>(mut buf: I) -> usize
where
    I: BufRead,
{
    let mut inp = Vec::<u8>::new();
    let _res = buf.read_until(10u8, &mut inp);
    let mut out = react_polymer(&inp);

    // last byte is \n, so pop
    assert_eq!(out.pop(), Some(10i8));
    out.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_a() {
        let input = b"dabAcCaCBAcCcaDA\n";
        assert_eq!(10, five_a(&input[..]));
    }

    #[test]
    fn test_five_b() {
        let input = b"dabAcCaCBAcCcaDA\n";
        assert_eq!(4, five_b(&input[..]));
    }
}
