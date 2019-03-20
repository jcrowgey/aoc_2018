use std::io::BufRead;
use std::ops::{Add, Rem};
use crate::finger::List;

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

fn walk_to(list: &mut List<i32>, new_index: i32, mut current_index: i32) -> i32 {
    while new_index != current_index {
        if new_index < current_index {
            list.go_left();
            current_index -= 1;
        }
        if new_index > current_index {
            list.go_right();
            current_index += 1;
        }
    };
    current_index
}

fn get_high_score_l(player_cnt: i32, max_turns: i32) -> i64 {
    let mut board = List::new();
    let mut board_len = 0;
    let mut player_scores = vec![0i64; player_cnt as usize];
    let mut last_index: i32 = 1;

    for i in 0..max_turns + 1 {
        if i == 0 || i == 1 {
            board.push_left(i);
            board_len += 1;
            last_index += i;
        } else {
            if i % 23 == 0 {
                let player = (i % player_cnt) as usize;
                player_scores[player] += i as i64;

                let mut remove_idx =
                    modulus(last_index - ELFLY_CONSTANT, board_len);

                if remove_idx == 0 {
                    // swing around to the right side because we're going to pop left below
                    remove_idx = board_len;
                }

                last_index = walk_to(&mut board, remove_idx, last_index);
                player_scores[player] += board.pop_left().unwrap() as i64;
                board_len -= 1;

                if last_index == (1 + board_len) { // we need to swing back again to walk right
                    last_index -= 1; // we're actually off by 1 since we popped the left stack
                    last_index = walk_to(&mut board, 1, last_index);
                } else {
                    board.go_right();  // the easy way to get back on track;
                }
            } else {
                // last index is already pointing to the right of the last insertion point
                let new_index = modulus(last_index + 1, board_len);
                last_index = walk_to(&mut board, new_index, last_index);
                board.push_left(i);
                board_len += 1;
                last_index += 1;
            }
        }
    }
    player_scores.sort();
    player_scores[player_scores.len() - 1]
}

fn get_high_score(player_cnt: i32, max_turns: i32) -> i32 {
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

pub fn nine_b<I>(buf: I) -> i64
where
    I: BufRead,
{
    let (player_cnt, max_turns) = param_extraction(buf);
    get_high_score_l(player_cnt, max_turns * 100)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUTS: [(&[u8; 45], i32); 6] = [
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

    #[test]
    fn test_walk_to() {
        let mut l = List::new();
        let mut i = 0;
        l.push_right(3);
        l.push_right(2);
        l.push_right(1);

        i = walk_to(&mut l, 2, i);
        assert_eq!(l.pop_right(), Some(3));
    }
}
