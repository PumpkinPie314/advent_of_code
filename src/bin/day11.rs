use std::{collections::HashMap, mem::take};
use itertools::Itertools;

fn main() {
    let input = include_str!("inputs/day11");
    // let input = "125 17 ";
    let  stones: Vec<u128> = input.split_whitespace().map(|x|x.parse().unwrap()).collect();
    // //p1
    // for i in 0..25 {
    //     stones = blink(stones);
    // };
    // println!("{:?}", stones.len());

    //p2
    let mut map: HashMap<u128, usize> = stones.into_iter().counts();
    let mut next_map: HashMap<u128, usize> = HashMap::new();
    for _ in 0..75  {
        for (engraving, occurrences) in map {
            let children = blink(vec![engraving]);
            for child in children {
                next_map.entry(child).and_modify(|v|*v+=occurrences).or_insert(occurrences);
            }
        }
        map = take(&mut next_map);
    }
    println!("{:?}", &map.values().sum::<usize>());


}
fn blink(stones: Vec<u128>) -> Vec<u128> {
    stones.into_iter().flat_map(|engraving|{
        if engraving == 0 {vec![1]}
        else if engraving.to_string().len() % 2 == 0 {
            let engraving = engraving.to_string();
            let (l,r ) = engraving.split_at(engraving.len()/2);
            vec![l.parse().unwrap() , r.parse().unwrap()]
        }
        else {
            vec![engraving*2024]
        }

    }).collect::<Vec<_>>()
}