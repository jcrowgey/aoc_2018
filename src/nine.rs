use std::collections::VecDeque;
use std::io::BufRead;

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

fn get_high_score(player_cnt: i32, max_turns: i32) -> i64 {
    let mut board = VecDeque::new();
    let mut player_scores = vec![0i64; player_cnt as usize];

    for i in 0..max_turns as i64 + 1 {
        if i == 0 {
            board.push_front(0);
        } else {
            if i % 23 == 0 {
                let player = (i % player_cnt as i64) as usize;
                player_scores[player] += i;
                for _ in 0..6 {
                    if let Some(popped_back) = board.pop_back() {
                        board.push_front(popped_back);
                    }
                }
                // rotate left 7 spots
                let popped = board.pop_back().unwrap();
                player_scores[player] += popped;
            } else {
                // rotate right by 2,
                for _ in 0..2 {
                    if let Some(popped_front) = board.pop_front() {
                        board.push_back(popped_front);
                    }
                }
                board.push_front(i);
            }
        }
    }
    player_scores.sort();
    player_scores[player_scores.len() - 1]
}

pub fn nine_a<I>(buf: I) -> i64
where
    I: BufRead,
{
    let (player_cnt, max_turns) = param_extraction(buf);
    // println!("{}, {}", player_cnt, max_turns);
    get_high_score(player_cnt, max_turns)
}

pub fn nine_b<I>(buf: I) -> i64
where
    I: BufRead,
{
    let (player_cnt, max_turns) = param_extraction(buf);
    get_high_score(player_cnt, max_turns * 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUTS: [(&[u8; 45], i64); 6] = [
        (b"09 players; last marble is worth 0026 points\n", 32),
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
