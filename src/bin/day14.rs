use std::{thread::sleep, time::Duration};

use glam::IVec2;
use itertools::Itertools;

fn main() {
    // let input = include_str!("tests/day14");
    // let bathroom_size = IVec2 {x: 11, y: 7};
    let input = include_str!("inputs/day14");
    let bathroom_size = IVec2 {x: 101, y: 103};
    let robots = input.lines()
        .flat_map(|l|l.split(' '))
        .flat_map(|pv|pv.split(','))
        .map(|n|n.chars().filter(|&x|x.is_digit(10) || x == '-').collect::<String>())
        .map(|n|n.parse::<i32>().unwrap())
        .tuples::<(i32, i32, i32, i32)>()
        .map(|(x, y, dx, dy)| (IVec2 {x, y}, IVec2 {x: dx, y: dy}))
        .collect::<Vec<_>>();
    let p1 = robots.clone().into_iter()
        .map(|robot|{
            // fast forward robots
            let (pos, vel) = robot;
            (pos + vel * 100).rem_euclid(bathroom_size)
        })
        // remove middle robots
        .filter(|pos| pos.x != bathroom_size.x/2 && pos.y != bathroom_size.y/2)
        .map(|pos|
            // produces a unique output for every quadrent 
            ((pos.x - bathroom_size.x/2).is_positive(), (pos.y-bathroom_size.y/2).is_positive())
        )
        .counts()
        .values()
        .product::<usize>();
    println!("{:?}", p1);
    for sec in 85..160 {
        let mut display = vec![vec!['.'; bathroom_size.x as usize]; bathroom_size.y as usize];
        for pos in robots.clone().into_iter().map(|r|(r.0 + r.1 * sec).rem_euclid(bathroom_size)) {
            display[pos.y as usize][pos.x as usize] = '#'
        }

        for line in display {
            println!("{}", line.iter().collect::<String>())
        }
        println!("seconds: {sec}\n");
        sleep(Duration::from_secs_f32(0.5));
    }

}