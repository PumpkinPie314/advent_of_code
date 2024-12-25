use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

fn main() {
    let input = include_str!("tests/day16");
    let mut is_wall: Vec<Vec<bool>> = vec![];
    let mut start_pos = (0, 0, 0, 0);
    let mut end_pos= (0, 0);
    for (r, l) in input.lines().enumerate() {
        is_wall.push(vec![]);
        for (c, char) in l.chars().enumerate() {
            if char == '#' {is_wall[r].push(true);}
            else if char == '.' {is_wall[r].push(false);}
            else if char == 'S' {start_pos = (r, c, 0, 1)}
            else if char == 'E' {end_pos = (r, c)}
        }
    }
    // dijktras using priority queue
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(Reverse((0usize, start_pos)));
    seen.insert(start_pos);
    while !queue.is_empty() {
        let current = queue.pop().unwrap().0;
        let (dist, (r, c, dr, dc)) = current;
        if end_pos == (r, c) {
            println!("{}", dist);
            break;
        }
        println!("{},{}", r, c);
        seen.insert((r, c, dr, dc));
        
        for neighbor in neighbors(current) {
            let (_dist, (r, c, dr, dc)) = neighbor;
            if is_wall[r][c] {continue;}
            if seen.contains(&(r, c, dr, dc)) {continue;}
            queue.push(Reverse(neighbor));
        }
    }
    
}
fn neighbors(current:(usize, (usize, usize, isize, isize))) -> impl Iterator<Item =(usize, (usize, usize, isize, isize))> {
    let (dist, (r, c, dr, dc)) = current;
    [
        (dist + 1, (r.checked_add_signed(dr).unwrap(), c.checked_add_signed(dc).unwrap(), dr, dc)),
        (dist + 1000, (r, c, -dc, dr)), // notice, dr and cd swap!
        (dist + 1000, (r, c, dc, -dr)),
    ].into_iter()
}