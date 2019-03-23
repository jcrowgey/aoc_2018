use std::io::BufRead;
use std::{thread, time};

fn parse_values(mut line: String) -> (i32, i32, i32, i32)
{
    let l_off = line.find("<").unwrap();
    let comma_off = line.find(",").unwrap();
    let r_off = line.find(">").unwrap();
    let x = line[l_off + 1..comma_off].trim().parse::<i32>().expect("problem parsing x");
    let y = line[comma_off + 1..r_off].trim().parse::<i32>().expect("problem parsing y");
    line.drain(..r_off + 1);

    let l_off = line.find("<").unwrap();
    let comma_off = line.find(",").unwrap();
    let r_off = line.find(">").unwrap();
    let xv = line[l_off + 1..comma_off].trim().parse::<i32>().expect("problem parsing xv");
    let yv = line[comma_off + 1..r_off].trim().parse::<i32>().expect("problem parsing yv");
    (x, y, xv, yv)
}

fn update(points: &mut Vec<(i32, i32, i32, i32)>) -> (i32, i32)
{
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for p in points.iter_mut() {
        (*p).0 = (*p).0 + (*p).2;
        (*p).1 = (*p).1 + (*p).3;
        if (*p).0 > max_x {
            max_x = (*p).0;
        }
        if (*p).1 > max_y {
            max_y = (*p).1;
        }
        if (*p).0 < min_x {
            min_x = (*p).0;
        }
        if (*p).1 < min_y {
            min_y = (*p).1;
        }
    }
    (max_x - min_x, max_y - min_y)
}

fn display(points: &mut Vec<(i32, i32, i32, i32)>)
{
    let mut to_draw = points.clone();
    to_draw.sort_by(|a, b| a.0.cmp(&b.0));
    let xmin = to_draw[0].0;
    let mut x = xmin;
    to_draw.sort_by(|a, b| a.1.cmp(&b.1));
    let mut y = to_draw[0].1;
    to_draw.reverse();
    // println!("{} {}", x, y);
    // println!("{:?}", to_draw);

    while to_draw.len() > 0 {
        let next = to_draw.pop().unwrap();
        while y < next.1 {
             println!();
             y += 1;
             x = xmin;
         }

         while x < next.0 {
             print!(" ");
             x += 1;
         }

         if x == next.0 {
             print!("#");
             x += 1;
         }
    }
    println!("\n\n");
}

pub fn ten<I>(buf: I)
where
    I: BufRead,
{

    let mut points = Vec::new();

    for line in buf.lines() {
        let p: (i32, i32, i32, i32) = parse_values(line.unwrap());
        points.push(p);
    }

    let mut i = 0;
    loop {
        let (x_delta, _) = update(&mut points);
        i += 1;
        if x_delta < 200 {
            display(&mut points);
            println!("{}", i);
            thread::sleep(time::Duration::from_millis(1000));
        }
    }
}
