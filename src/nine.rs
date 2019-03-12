use std::io::BufRead;
use std::ops::{Add, Rem};

static ELFLY_CONSTANT: i32 = 7;

fn modulus<T>(a: T, b: T) -> T
where
    T: Add<Output = T> + Rem<Output = T> + Copy,
{
    ((a % b) + b) % b
}

fn param_extraction<I>(mut buf: I) -> (i32, i32)
where
    I: BufRead,
{
    let mut input = Vec::<u8>::new();

    let _read = buf.read_until(10u8, &mut input);
    let mut input = String::from_utf8(input).unwrap();
    let players: String = input.drain(..input.find(" players;").unwrap()).collect();
    let players: i32 = players.parse().unwrap();

    let worth_idx = input.find("worth ").unwrap();
    let _text: String = input.drain(..worth_idx + 6).collect();
    let points_idx = input.find(" points").unwrap();
    let points: String = input.drain(..points_idx).collect();
    let points: i32 = points.parse().unwrap();

    (players, points)
}

fn get_high_score(player_cnt: i32, max_turns: i32) -> i32 {
    // let mut board = Vec::with_capacity(max_turns as usize);
    let mut board = Vec::new();
    let mut player_scores = vec![0i32; player_cnt as usize];
    let mut last_index: usize = 0;

    for i in 0..max_turns + 1 {
        if i == 0 {
            board.push(0);
        } else {
            if i % 23 == 0 {
                let player = (i % player_cnt) as usize;
                player_scores[player] += i;
                let remove_idx =
                    modulus(last_index as i32 - ELFLY_CONSTANT, board.len() as i32) as usize;
                player_scores[player] += board.remove(remove_idx);
                last_index = modulus(remove_idx, board.len());
            } else {
                let new_index = modulus(last_index + 2, board.len());
                board.insert(new_index, i);
                last_index = new_index;
            }
        }
    }
    player_scores.sort();
    player_scores[player_scores.len() - 1]
}

pub fn nine_a<I>(buf: I) -> i32
where
    I: BufRead,
{
    let (player_cnt, max_turns) = param_extraction(buf);
    // println!("{}, {}", player_cnt, max_turns);
    get_high_score(player_cnt, max_turns)
}

pub fn nine_b<I>(buf: I) -> i32
where
    I: BufRead,
{
    let (player_cnt, max_turns) = param_extraction(buf);
    get_high_score(player_cnt, max_turns * 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUTS: [(&[u8; 45], i32); 5] = [
        (b"10 players; last marble is worth 1618 points\n", 8317),
        (b"13 players; last marble is worth 7999 points\n", 146373),
        (b"17 players; last marble is worth 1104 points\n", 2764),
        (b"21 players; last marble is worth 6111 points\n", 54718),
        (b"30 players; last marble is worth 5807 points\n", 37305),
    ];

    #[test]
    fn test_nine_a() {
        for (input, expected) in INPUTS.iter() {
            assert_eq!(*expected, nine_a(&input[..]));
        }
    }
}
