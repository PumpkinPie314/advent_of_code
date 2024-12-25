use std::collections::HashSet;

use itertools::Itertools;
use glam::IVec2;

use advent_of_code::grid::Grid;

fn main() {
    // let input = include_str!("tests/day8");
    let input = include_str!("inputs/day8.txt");
    let grid = Grid::from_str(input);
    let frequencys = input.chars().filter(|&c| !c.is_whitespace() && c != '.').unique().collect::<Vec<_>>(); 

    let mut p1: HashSet<IVec2> = HashSet::new();
    let mut p2: HashSet<IVec2> = HashSet::new();
    for freq in frequencys {
        let towers: Vec<IVec2> = grid.cells_enumerate().filter(|&(_, &c)| c == freq).map(|(pos, _)|pos).collect();
        // // a blank grid I right to, for debuging
        // let mut display = Grid::new_from(vec![vec!['.'; grid.width()]; grid.height()]);
        // for &t in &towers {
        //     display.set_cell(t, freq);
        // }
        for tower_pair in towers.into_iter().combinations(2).collect::<Vec<_>>() {
            let (a, b) = (tower_pair[0], tower_pair[1]);
            // p1
            for antinode in [a+(a-b), b-(a-b)] {
                if grid.contains_coords(antinode) {
                    p1.insert(antinode);
                }
            }
            // p2
            let mut pointer = a.clone();
            while grid.contains_coords(pointer) {
                // display.set_cell(pointer, '#');
                p2.insert(pointer);
                pointer -= a-b;
                // println!("p2: {:?}", p2.len());
                // println!("");
            }
            pointer = b.clone();
            while grid.contains_coords(pointer) {
                // display.set_cell(pointer, '#');
                p2.insert(pointer);
                pointer += a-b;
            };
        }
    }
    println!("p1: {:?}", p1.len());
    println!("p2: {:?}", p2.len());
}

