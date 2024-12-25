use advent_of_code::grid::Grid;
use glam::IVec2;

fn main() {
    // let input = include_str!("tests/day15");
    let input = include_str!("inputs/day15");
    let lines = input.lines().collect::<Vec<_>>();
    let mut input = lines
        .split(|x|x.is_empty())
        .map(|x|x);
    let top_half = input.next().unwrap();
    let bottom_half = input.next().unwrap().concat();

    let mut grid = Grid::from_str(top_half.join("\n").as_str());
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

        let mut push_affected = vec![];
        let mut pressure = robot;
        let should_push =  loop {
            push_affected.push(pressure);
            pressure += dir;
            match *grid.get(pressure).unwrap() {
                '.' => {break true},
                '#' => {break false},
                'O' => {continue;}
                _ => panic!()
            }
        };
        // println!("{:?}", push_affected.iter().map(|&p|grid.get(p).unwrap()).collect::<Vec<_>>());
        if should_push {
            for affected in push_affected.into_iter().rev() {
                grid.set_cell(affected + dir, *grid.get(affected).unwrap());
                grid.set_cell(affected, '.');
            }
            robot += dir
        };
    };
    let sum = grid.cells_enumerate()
        .filter(|(_, c)| **c == 'O')
        .map(|(p, _)| p.x + 100*p.y)
        .sum::<i32>();
    println!("{}", grid.clone().to_string());
    println!("{}", sum);
}