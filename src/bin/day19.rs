use std::collections::HashMap;

fn main() {
    // let input = include_str!("tests/day19");
    let input = include_str!("inputs/day19");
    let mut lines  = input.lines();
    let mut towels = lines.next().unwrap().split(", ").collect::<Vec<_>>();
    towels.sort_by(|a, b|a.len().cmp(&b.len()));
    lines.next();
    let designs = lines.collect::<Vec<_>>();
    let mut memo:HashMap<&str, usize> = HashMap::new();

    for d in &designs {
        println!("{}: {}", d, num_ways(&towels, d, &mut memo));
    }
    let p1 = designs.iter().map(|d|num_ways(&towels, d, &mut memo) > 0).filter(|b|*b).count();
    let p2 = designs.iter().map(|d|num_ways(&towels, d, &mut memo)).sum::<usize>();
    println!("p1 : {}\tp2: {}", p1, p2)
}

fn num_ways<'a>(towels: &Vec<&str>, design: &'a str, memo: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() { return 1 }
    if let Some(ways) = memo.get(design) {
        return *ways
    }
    let mut count = 0; 
    for t in towels {
        if let Some(desigin_left) = design.strip_prefix(t) {
            count += num_ways(towels, desigin_left, memo)
        }
    };
    memo.insert(design, count);
    count
}