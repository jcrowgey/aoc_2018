use std::io::BufRead;
use std::collections::{HashMap, HashSet};

fn taxi_distance(x1: i32, y1: i32, x2: i32, y2:i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}


fn parse_point(mut line: String) -> (i32, i32) {
    let x: String = line.drain(..line.find(",").unwrap()).collect();
    let x = x.parse::<i32>().expect("problem parsing x");
    let y = line.trim_start_matches(", ")
                .trim_end()
                .parse::<i32>().expect("problem parsing y");
    (x, y)
}

fn find_closest<'a, I>(x: i32, y: i32, points: I) -> Option<(i32, i32)>
where
    I: Iterator<Item=&'a (i32, i32)>
{
    let mut closest_point: (i32, i32) = (-1, -1);
    let mut best_distance: i32 = 0x7fffffff;
    let mut tie = false;

    for (px, py) in points {
        let d = taxi_distance(x, y, *px, *py);
        if d == best_distance {
            tie = true;
        }
        else if d < best_distance {
            best_distance = d;
            closest_point = (*px, *py);
            tie = false;
        }
    }

    match tie {
        false => Some(closest_point),
        true => None,
    }
}

// holds the bounding box and the points of interest
struct SpaceMap {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    points: HashMap<(i32, i32), i32>,
}

impl SpaceMap {
    fn new<I>(buf: I) -> SpaceMap
    where I:
        BufRead
    {
        let mut max_x: i32 = 0;
        let mut max_y: i32 = 0;
        let mut min_x: i32 = 0x7fffffff;
        let mut min_y: i32 = 0x7fffffff;
        let mut points = HashMap::new();

        for line in buf.lines() {
            let (x, y) = parse_point(line.unwrap());
            points.insert((x, y), 0);

            if x < min_x {
                min_x = x;
            }

            if x > max_x {
                max_x = x;
            }

            if y < min_y {
                min_y = y;
            }

            if y > max_y {
                max_y = y;
            }
        }

        SpaceMap {
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
            points: points,
        }
    }
}

/*
 * naive approach:
 * check every point on the bounding box edge, note all points that one on of these spots
 * check every point within the bounding box and keep score
 * take the item with the max score which isn't in the set to exclude
 *
 * naive implementation
 * 1. pass through the input, find the bounding box
 * 2. pass around the box edge, rule out owners
 * 2. pass through every point in the box, compute its closest (passing through every nonexcluded
 *    input), keep score
 * 3. return the best score
 */
pub fn six_a<I>(buf: I) -> i32
where
    I: BufRead
{
    let mut space = SpaceMap::new(buf);

    // pass around the box edge and note excluded owners
    let mut excluded = HashSet::new();
    for x in space.min_x..space.max_x {
        match find_closest(x, space.min_y, space.points.keys()) {
            Some(cp) => excluded.insert(cp),
            None => false,
        };

        match find_closest(x, space.max_y, space.points.keys()) {
            Some(cp) => excluded.insert(cp),
            None => false,
        };
    }

    for y in space.min_y..space.max_y {
        match find_closest(space.min_x, y, space.points.keys()) {
            Some(cp) => excluded.insert(cp),
            None => false,
        };

        match find_closest(space.max_x, y, space.points.keys()) {
            Some(cp) => excluded.insert(cp),
            None => false,
        };
    }

    // add up the score for all points
    for x in space.min_x .. space.max_x {
        for y in space.min_y .. space.max_y {
            match find_closest(x, y, space.points.keys()) {
                Some(cp) => {
                    *space.points.entry(cp).or_insert(0) += 1;
                },
                None => (),
            };
        }
    }

    for p in excluded.iter() {
        space.points.remove(&p);
    }

    *space.points.values().max().unwrap()
}


pub fn six_b<I>(buf: I, threshold: i32) -> usize
where
    I: BufRead
{

    let space = SpaceMap::new(buf);
    let mut size = 0;

    for x in space.min_x .. space.max_x {
        for y in space.min_y .. space.max_y {
            let sum = space.points.keys()
                                  .fold(0, |acc, (px, py)| acc + taxi_distance(x, y, *px, *py));
            if sum < threshold {
                size += 1;
            }
        }
    }

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &[u8; 30] = b"1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
";

    #[test]
    fn test_six_a() {
        assert_eq!(17, six_a(&INPUT[..]));
    }

    #[test]
    fn test_six_b() {
        let thresh = 32;
        assert_eq!(16, six_b(&INPUT[..], thresh));
    }

    #[test]
    fn test_parse_point() {
        let mut str_pair = "11, 99\n".to_string();
        assert_eq!((11i32, 99i32), parse_point(str_pair));
    }
}
