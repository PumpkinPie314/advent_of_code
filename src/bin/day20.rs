use std::collections::{HashMap, VecDeque};

use advent_of_code::grid::Grid;
use glam::IVec2;
use itertools::Itertools;

fn main() {
    // let input = include_str!("tests/day20");
    let input = include_str!("inputs/day20");
    let grid = Grid::from_str(input);
    let _start = grid.find_position_first(|c|*c == 'S').unwrap();
    let exit = grid.find_position_first(|c|*c == 'E').unwrap();


    //bfs
    let mut queue = VecDeque::new();
    let mut distances: HashMap<IVec2, usize> = HashMap::new();
    let mut backtrack: HashMap<IVec2, Option<IVec2>> = HashMap::new();
    queue.push_back((exit, 0usize));
    distances.insert(exit, 0);
    backtrack.insert(exit, None);
    while let Some((current, dist)) = queue.pop_front(){
        for adj in grid.adjacent_cells(current) {
            if distances.contains_key(&adj) {continue;}
            match grid.get(adj) {
                Some('.') | Some ('S') => {
                    distances.insert(adj, dist+1);
                    backtrack.insert(adj, Some(current));
                    queue.push_back((adj, dist+1));
                }
                Some('#') | None => {}
                _ => unreachable!()
            }
        }
    }
    // // p1
    // let two_away = [
    //                  (-2, 0),
    //           (-1,-1),      (-1, 1),
    //     (0,-2),                    (0,2),
    //           ( 1,-1),      ( 1, 1),
    //                  ( 2, 0),
    // ].into_iter().map(|(x, y)|IVec2 {x, y});

    // p2
    let up_to_twenty_away = (-20..=20i32).cartesian_product(-20..=20i32)
        .filter(|&(x, y)| x.abs()+y.abs() <= 20)
        .map(|(x, y)|IVec2 {x, y});

    // takes cheat_start and cheat_end and returns the time saved
    let mut possible_time_saved: HashMap<(IVec2, IVec2), usize> = HashMap::new();
    for &cheat_start in distances.keys() {
        // p1
        // for delta in two_away.clone() {
        for delta in up_to_twenty_away.clone() {
            let cheat_end = cheat_start + delta;
            // make sure the place we are phasing to can reach the exit
            if !&distances.contains_key(&cheat_end) {continue;}
            if let Some(mut time_saved) = distances.get(&cheat_start).unwrap().checked_sub(*distances.get(&cheat_end).unwrap()) {
                time_saved -= delta.abs().element_sum() as usize; // 
                if time_saved <= 0 {continue;}
                possible_time_saved.insert((cheat_start, cheat_end), time_saved);
            };
        }
    }

    // for ((cheat_start, cheat_end), time_saved) in possible_time_saved.clone().into_iter() {
    //     if time_saved != 12  {continue;}
    //     println!("{}",distances.get(&cheat_start).unwrap());
    //     println!("{}",distances.get(&cheat_end).unwrap());
    //     println!("{}\n", grid.clone_map_enumerate(|(pos,c)|{
    //         if cheat_start == pos {return '0'}
    //         else if cheat_end == pos {return  '2'}
    //         else {*c}
    //     }).to_string());
    // }

    let p2 = possible_time_saved.values()
        .counts()
        .into_iter()
        .filter(|(time_saved, _count)|{
            **time_saved >= 100
        })
        .map(|(_time_saved, count)|{
            count
        }).sum::<usize>()
        ;
    
    // for (k,v) in p2.into_iter() {
    //     println!("There are {v} cheats that save {k} picoseconds.")
    // }
    println!("{:?}", p2)

}