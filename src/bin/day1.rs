use itertools::Itertools;

fn main() {
    // let input = 
    // "3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3";
    let input = include_str!("inputs/day1");

    let (mut left, mut right) : (Vec<i32>, Vec<i32>)= input.lines().map(|line|{
        let mut words = line.split_whitespace();
        let left = words.next().unwrap().parse::<i32>().unwrap();
        let right = words.next().unwrap().parse::<i32>().unwrap();
        (left, right)
    }).unzip();

    // // part one
    left.sort();
    right.sort();

    let p1: i32 = left.iter().zip(right.iter()).map(|(l,r)|(l-r).abs()).sum();

    // part two
    
    let right_counts = right.into_iter().counts();
    let p2 = left.iter().fold(0, |acc, x|{
        acc + x * *right_counts.get(x).unwrap_or(&0) as i32
    });
    println!("p1: {}, p2: {}", p1, p2)

}
