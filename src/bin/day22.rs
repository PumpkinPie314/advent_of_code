use std::collections::HashMap;
use itertools::iterate;

fn main() {
    // let input = include_str!("tests/day22");
    let input = include_str!("inputs/day22");
    let input = input.lines().map(|x|x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let p1 = input.clone().into_iter()
        .map(|seed|iterate(seed, next_secret_number).nth(2000).unwrap())
        .sum::<usize>();
    println!("{:?}", p1);

    let all_maps = input.into_iter().map(|seed|{
        let mut price_from_sequence: HashMap<(i8, i8, i8, i8), i8> = HashMap::new();
        let mut secret = next_secret_number(&seed);
        let mut prev_changes = vec![];
        let mut prev_price = (seed%10) as i8;
        for _ in 0..2000 {
            let price = (secret%10) as i8;
            let change = price - prev_price;

            prev_changes.insert(0, change);
            if prev_changes.len() == 4 {
                let sequence = (prev_changes[3], prev_changes[2],prev_changes[1],prev_changes[0]);
                price_from_sequence.entry(sequence).or_insert(price);// doesn't insert price if ones already there
                prev_changes.pop();//forget
            }
            prev_price = price;
            secret = next_secret_number(&secret)
        }
        price_from_sequence
    });
    // combine all the maps!
    let mut master_map = HashMap::new();
    for map in all_maps {
        for (sequence, price) in map {
            *master_map.entry(sequence).or_insert(0) += price as usize;
        }
    }
    let p2 = master_map.values().max().unwrap();
    println!("{:?}", p2)

}
const PRUNE: usize = 16777215; // this usize has a 1 in the rightmost 24 bits, we use this as a bitmask to get modulus 2^24
fn next_secret_number(n: &usize) -> usize {
    let mut out = (n << 6 ^ n) & PRUNE;
    out = (out >> 5 ^ out) & PRUNE;
    (out << 11 ^ out) & PRUNE
}