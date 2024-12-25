use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

fn main() {
    let input = include_str!("tests/day24");
    // let input = include_str!("inputs/day24");
    let mut input = input.lines();
    let mut wires = input
        .take_while_ref(|l|!l.is_empty())
        .map(|l|{
            let mut s = l.split(": ");
            let wire = s.next().unwrap();
            let value = s.next().unwrap() != "0";
            (wire, value)
        })
        .collect::<HashMap<_, _>>();
    input.next();
    let mut gates: VecDeque<(&str, &str, &str, &str)> = input.map(|l|{
        let mut l = l.split_whitespace();
        let wire1 = l.next().unwrap();
        let gate = l.next().unwrap();
        let wire2 = l.next().unwrap();
        l.next(); // arrow
        let wire_out = l.next().unwrap();
        (wire1, gate, wire2, wire_out)
    }).collect();
    loop {
        if gates.is_empty() {break;}
        let (wire1, gate, wire2, wire_out) = gates.pop_front().unwrap();
        if wires.contains_key(wire1) && wires.contains_key(wire2) {
            let lhs = *wires.get(wire1).unwrap();
            let rhs = *wires.get(wire2).unwrap();
            let val = match gate {
                "AND" => {
                    lhs & rhs
                }
                "OR" => {
                    lhs | rhs
                }
                "XOR" => {
                    lhs ^ rhs
                }
                _ => panic!()
            };
            wires.insert(wire_out, val);
        } else {
            gates.push_back((wire1, gate, wire2, wire_out));
        }
    }
    let mut wires=  wires.into_iter().collect::<Vec<_>>();
    wires.sort();
    let mut wires =  wires.into_iter().rev();
    let (mut x, mut y, mut z) = (0usize, 0usize, 0usize);
    while let Some((wire, value)) = wires.next() {
        let starts_with = wire.chars().next().unwrap();
        match starts_with {
            'x' => {x = (x << 1) | (value as usize)}
            'y' => {y = (y << 1) | (value as usize)}
            'z' => {z = (z << 1) | (value as usize)}
            _ => {}
        } 
    }

    println!("{} + {} = {}", x, y, z)
        
}