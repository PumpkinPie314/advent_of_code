use std::collections::VecDeque;

use advent_of_code::grid::Grid;
use glam::IVec2;

fn main() {
    // let input = include_str!("tests/day15");
    let input = include_str!("inputs/day15");
    let lines = input.lines().collect::<Vec<_>>();
    let mut input = lines
        .split(|x|x.is_empty());
        // .map(|x|x);
    let top_half = input.next().unwrap();
    let bottom_half = input.next().unwrap().concat();

    let grid = top_half.join("\n").chars().flat_map(|c|{
        match c {
            '\n' => {vec!['\n']}
            '@' => {vec!['@', '.']}
            'O' => {vec!['[', ']']}
            c => {vec![c, c]}
        }
    }).collect::<String>();
    let mut grid = Grid::from_str(grid.as_str());
    let directions = bottom_half.chars()
    .map(|c|{
        let dir = match c {
            '^' => {(0, -1)},
            'v' => {(0, 1)},
            '>' => {(1, 0)},
            '<' => {(-1, 0)},
            _ => panic!("not a direction")
        };
        IVec2::from(dir)
    });
    println!("{}", grid.clone().to_string());
    let mut robot = grid.find_position_first(|&c|c == '@').expect("no robot");
    for dir in directions.into_iter() {
        assert!(*grid.get(robot).unwrap() == '@');
        // println!("dir: {:?}", dir);

        let mut queue = VecDeque::new();
        let mut may_push = vec![];
        let mut should_push = true;
        queue.push_back(robot);
        while !queue.is_empty() && should_push {
            // println!("{:?}", queue);
            // println!("{:?}", queue.iter().map(|&p|grid.get(p).unwrap()).collect::<Vec<_>>());
            let pressure_wave = queue.pop_front().unwrap();
            if may_push.contains(&pressure_wave) {continue;}
            match *grid.get(pressure_wave).unwrap() {
                '@' | 'O' => {}
                '[' => {
                    queue.push_back(pressure_wave + IVec2 {x: 1, y:0});
                }
                ']' => {
                    queue.push_back(pressure_wave + IVec2 {x:-1, y:0});
                }
                '#' => {
                    should_push = false;
                    break;
                }
                '.' => {continue;},
                _ => panic!(),
            }
            may_push.push(pressure_wave);
            queue.push_back(pressure_wave + dir);
        } 
        // println!("{:?}", may_push);






        if should_push {
            for affected in may_push.into_iter().rev() {
                grid.set_cell(affected + dir, *grid.get(affected).unwrap());
                grid.set_cell(affected, '.');
            }
            robot += dir
        };
    };
    let sum = grid.cells_enumerate()
        .filter(|(_, c)| **c == 'O' || **c == '[')
        .map(|(p, _)| p.x + 100*p.y)
        .sum::<i32>();
    println!("{}", grid.clone().to_string());
    println!("{}", sum);
}