use std::collections::HashMap;

use glam::IVec2;

const NUMERIC : [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['p', '0', 'A'],
];
const DIRECTIONAL: [[char; 3]; 2] = [
    ['p', '^', 'A'],
    ['<', 'v', '>'],
];
fn main() {
    let input = include_str!("tests/day21");
    let door_codes = input.lines().collect::<Vec<_>>();
    
    let mut numpad_memo = HashMap::new();
    let mut direction_pad_memo = HashMap::new();
    let mut proccess = door_codes.iter().map(|door_code|{
        let mut code_digits = door_code.chars().collect::<Vec<_>>();
        code_digits.insert(0, 'A');
        let mut out = String::new();
        for i in 0..code_digits.len()-1 {
            out.push_str(delta(&NUMERIC, &mut numpad_memo, code_digits[i], code_digits[i+1]).as_str());
        }
        out
    }).collect::<Vec<_>>();
    println!("{:?}", proccess);
    for _ in 0..2 {
        proccess = proccess.into_iter().map(|door_code|{
            let mut code_digits = door_code.chars().collect::<Vec<_>>();
            code_digits.insert(0, 'A');
            let mut out = String::new();
            for i in 0..code_digits.len()-1 {
                out.push_str(delta(&DIRECTIONAL, &mut direction_pad_memo, code_digits[i], code_digits[i+1]).as_str());
            }
            out
        }).collect::<Vec<_>>();
        println!("{:?}", proccess);
    }
    proccess = proccess.into_iter().zip(door_codes).map(|(o, dc)| format!("{} * {}", o.len(), dc)).collect();
    println!("{:?}", proccess);
    proccess = proccess.into_iter().map(|x| {
        x.chars()
            .filter(|c|c.is_digit(10) || *c == ' ' || *c == '*')
            .collect::<String>()
    }).collect();
    println!("{:?}", proccess);
    let out = proccess.into_iter().map(|l|{
        let mut opperands = l.split(" * ");
        let len = opperands.next().unwrap()
            .parse::<usize>()
            .unwrap();
        let code = opperands.next().unwrap()
            .parse::<usize>()
            .unwrap();
        len * code
    }).sum::<usize>();
    println!("{:?}", out);
    
}
fn delta(layout:& [[char; 3]], memo: & mut HashMap<(char, char), String>, from: char, to: char) -> String {
    if let Some(out) = memo.get(&(from, to)) {
        // println!("HIT! : {from}->{to}");
        return out.clone()
    }
    let mut from_pos = Some(IVec2::default());
    let mut to_pos = Some(IVec2::default());
    for (y, l) in layout.into_iter().enumerate() {
        for (x, c) in l.into_iter().enumerate() {
            if *c == from {from_pos = Some(IVec2 {x: x as i32, y: y as i32})}
            if *c == to   {to_pos   = Some(IVec2 {x: x as i32, y: y as i32})}
        }
    }
    if from_pos == None || to_pos == None {panic!("{} or {} are not real!", from, to)}
    let delta = to_pos.unwrap() - from_pos.unwrap();
    let mut out = String::new();
    for _ in 0..delta.abs().x {
        if delta.x.is_positive() {out.push('>');}
        if delta.x.is_negative() {out.push('<');}
    }
    for _ in 0..delta.abs().y {
        if delta.y.is_positive() {out.push('v');}
        if delta.y.is_negative() {out.push('^');}
    }
    out.push('A');
    memo.insert((from, to), out.clone());
    out
}