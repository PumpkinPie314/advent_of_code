use std::collections::{HashSet, VecDeque};

use advent_of_code::grid::Grid;
use glam::IVec2;
use itertools::Itertools;

fn main() {
    // let (input, size, take): (&str, (usize, usize), usize)= (include_str!("tests/day18"), (6, 6), 12);
    let (input, size, take): (&str, (usize, usize), usize)= (include_str!("inputs/day18"), (70, 70), 1024);
    let points = input.lines()
        .flat_map(|l|l.split(','))
        .map(|n|n.parse::<i32>().unwrap())
        .tuples()
        .map(|(x, y)| IVec2{x, y})
        .collect::<Vec<IVec2>>();
    let mut grid = Grid::new_from(vec![vec![false; size.0 + 1]; size.1 + 1]);
    for point in points.iter().take(take) {
        grid.set_cell(*point, true);
    }
    println!("{}", grid.clone().to_string());
    // bfs
    let mut queue: VecDeque<(IVec2, usize)> = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(((0,0).into(), 0));
    while !queue.is_empty() {
        let (current, dist) = queue.pop_front().unwrap();
        // println!("current: {current}\t queue: {:?}", &queue);
        if current == (size.0 as i32, size.1 as i32).into() {
            println!("p1: {dist}");
            break;
        }
        visited.insert(current);
        for adj in grid.adjacent_cells(current) {
            if !grid.contains_coords(adj) {continue;}
            if visited.contains(&adj) {continue;}
            if queue.iter().any(|(pos, _)|pos == &adj) {continue;}
            if grid.get_unchecked(adj) == true {continue;}//if corrupted
            queue.push_back((adj, dist + 1));
        }
    }
    let mut grid = Grid::new_from(vec![vec![false; size.0 + 1]; size.1 + 1]);
    for point in points.iter().skip(take) {
        grid.set_cell(*point, true);
        // println!("{:?}", point);
        if !can_exit(&grid) {
            // println!("{}", grid.clone().to_string());
            println!("p2: {point}");
            break;
        }
    }
}
// 206 is too low
fn can_exit(grid: &Grid<bool>) -> bool {
    let mut queue: VecDeque<IVec2> = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((0,0).into());
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current == (grid.width() as i32 - 1, grid.height() as i32 - 1).into() {
            // println!("success!: {}",current);
            return true
        }
        visited.insert(current);
        for adj in grid.adjacent_cells(current) {
            if !grid.contains_coords(adj) {continue;}
            if visited.contains(&adj) {continue;}
            if queue.contains(&adj) {continue;}
            if grid.get_unchecked(adj) == true {continue;}//if corrupted
            queue.push_back(adj);
        }
    }
    false
}