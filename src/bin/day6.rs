use std::{collections::{HashMap, HashSet}, ops::Not};
use glam::IVec2;

fn main() {
    // let input = include_str!("test");
    let input = include_str!("inputs/day6");
    let mut guard_start: IVec2 = IVec2::default();
    let mut obstical_positions: HashSet<IVec2> = HashSet::new();
    //parse
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {obstical_positions.insert(IVec2::from((x.try_into().unwrap(), y.try_into().unwrap())));}
            if c == '^' {guard_start = IVec2::from((x.try_into().unwrap(), y.try_into().unwrap()));}
        }
    }
    let room_size = IVec2::from((input.lines().next().unwrap().len().try_into().unwrap(), input.lines().count().try_into().unwrap()));

    let p1 = traverse(guard_start, &obstical_positions, room_size).visited_spaces().unwrap();
    println!("p1: {}", p1.len());

    let mut p2 = 0;
    for (n, space) in p1.iter().enumerate() {
        let mut alternate_obsticals = obstical_positions.clone();
        alternate_obsticals.insert(*space);
        if traverse(guard_start, &alternate_obsticals, room_size).escaped().not() {
            p2 += 1;
        }
        if n%100 == 0 {println!("{n}/{}", p1.len())}
    }
    println!("p2: {p2}")
}


#[derive(Debug)]
enum TraverseResault {
    Escaped(Vec<IVec2>),
    Loop
}
impl TraverseResault {
    fn escaped(&self) -> bool {
        match self {
            Self::Escaped(_) => {return true},
            Self::Loop => {return false}
        }
    }
    fn visited_spaces(&self) -> Option<Vec<IVec2>> {
        match self {
            Self::Escaped(x) => {return Some(x.clone())},
            Self::Loop => {return None}
        }
    }
}
fn traverse(guard_pos: IVec2, obstical_positions: &HashSet<IVec2>, room_size: IVec2) -> TraverseResault{
    let mut guard_pos = guard_pos;
    let mut direction: IVec2 = IVec2::from((0, -1));
    let mut visited: HashMap<IVec2, Vec<IVec2>> = HashMap::new();// remembers what tiles i've been in, and what direction I was going when I was there
    loop {
        visited.entry(guard_pos).or_default().push(direction);
        // if wall ahead
        if obstical_positions.contains(&(guard_pos + direction)) {
            // turn right
            direction = IVec2::new(-direction.y, direction.x)
        }
        guard_pos += direction;
        //stop when out of bounds
        if guard_pos.x.is_negative() ||
            guard_pos.y.is_negative() ||
            guard_pos.x >= room_size.x ||
            guard_pos.y >= room_size.y
        {
            return TraverseResault::Escaped(visited.keys().cloned().collect::<Vec<_>>());
        }
        // if I have been here before, then I am in a loop!
        if let Some(prev_directions) = visited.get(&guard_pos) {
            if prev_directions.contains(&direction) {
                return TraverseResault::Loop;
            }
        }
    }
}