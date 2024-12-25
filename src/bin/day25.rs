#![feature(portable_simd)]
use std::simd::u8x8;
use itertools::{self, Itertools};

fn main() {
    // let input = include_str!("tests/day25");
    let input = include_str!("inputs/day25");

    let lines = input.lines().collect::<Vec<_>>();
    let blueprints = lines.split(|x|x.is_empty()).collect::<Vec<_>>();

    let mut keys = vec![];
    let mut locks = vec![];
    for blueprint in blueprints {
        let is_key = blueprint[0] == "#####";
        let heights = blueprint.into_iter().map(|l|{
            let mut l = l.to_string();
            l.push_str("...");
            let x = l.chars().map(|c|(c=='#') as u8).collect::<Vec<_>>();
            let x: [u8; 8] = x.try_into().unwrap();
            let x : u8x8 = x.into();
            x
        }).sum::<u8x8>() - u8x8::from_array([1,1,1,1,1,0,0,0]);
        match is_key {
            true => keys.push(heights),
            false => locks.push(heights),
        };
    }
    let mut p1 = 0;
    for (lock_i, key_i) in (0..locks.len()).cartesian_product(0..keys.len()) {
        if can_fit(locks[lock_i], keys[key_i]) {p1 += 1}
    }
    println!("p1: {}", p1);
}
fn can_fit(lock: u8x8, key: u8x8) -> bool {
    *(lock + key).as_array().iter().max().unwrap() <= 5
}
//59173 too high