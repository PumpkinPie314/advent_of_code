use std::{cmp::Ordering, collections::{BTreeSet, HashSet}};

use advent_of_code::grid::Grid;
use glam::IVec2;

const DIRECTIONS: [IVec2; 4] = [
    IVec2{x:1, y:0},
    IVec2{x:0, y:1},
    IVec2{x:-1, y:0},
    IVec2{x:0, y:-1},
];

fn main() {
    let input = include_str!("tests/day16");
    let grid = Grid::from_str(input);
    let start_pos = grid.find_position_first(|c|*c=='S').unwrap();
    let exit_pos = grid.find_position_first(|c|*c=='E').unwrap();

    // dijkstra 
    let mut priority_queue: BTreeSet<Node> = BTreeSet::new();
    let mut visited: HashSet<(IVec2, u8)> = HashSet::new();
    priority_queue.insert(
        Node {
            grid: &grid,
            pos: start_pos,
            direction: 0,
            cost_from_start: 0,
            bread_crumbs: vec![],
        }
    );
    loop {
        let current_node: Node<'_> = priority_queue.pop_first().unwrap();
        current_node.println_debug();
        if current_node.pos == exit_pos {
            println!("p1 : {}", current_node.cost_from_start);
            return
        }
        visited.insert(current_node.pos_and_dir());
        for adj_node in current_node.get_adjacent() {
            println!("{:?} is adjacent to: {:?}\n", current_node.pos_and_dir(), adj_node.pos_and_dir());
            if visited.contains(&adj_node.pos_and_dir()) {continue;}
            priority_queue.insert(adj_node);
        }
    }
    
}

#[derive(Debug, Clone)]
struct Node<'a> {
    grid: &'a Grid<char>,
    pos: IVec2,
    direction: u8,
    cost_from_start: u32,
    bread_crumbs: Vec<IVec2>
}
impl <'a>Node<'a> {
    fn get_adjacent(&self) -> impl Iterator<Item = Node<'a>>{
        let mut adjacent = vec![];
        if self.grid.get_unchecked(self.pos + DIRECTIONS[self.direction as usize]) != '#'  {
            adjacent.push(Node {
                grid: self.grid,
                pos: self.pos + DIRECTIONS[self.direction as usize],
                direction: self.direction,
                cost_from_start: self.cost_from_start + 1,
                bread_crumbs: [self.bread_crumbs.clone(), vec![self.pos]].concat()
            });
        }
        adjacent.push(Node {
            grid: self.grid,
            pos: self.pos,
            direction: (self.direction + 1)%4,
            cost_from_start: self.cost_from_start + 1000,
            bread_crumbs: [self.bread_crumbs.clone(), vec![self.pos]].concat()
        });
        adjacent.push(Node {
            grid: self.grid,
            pos: self.pos,
            direction: (self.direction + 3)%4,
            cost_from_start: self.cost_from_start + 1000,
            bread_crumbs: [self.bread_crumbs.clone(), vec![self.pos]].concat()
        });
        adjacent.into_iter()
    }
    fn pos_and_dir(&self) -> (IVec2, u8) {
        (
            self.pos,
            self.direction
        )
    }
    fn println_debug(&self) {
        println!("pos: {}", self.pos);
        println!("direction: {}", self.direction);
        println!("cost_from_start: {}", self.cost_from_start);
        println!("bread_crumbs :{:?}", self.bread_crumbs);
        println!("");
    }
}
impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.cost_from_start == other.cost_from_start
    }
}
impl Eq for Node<'_> {}
impl PartialOrd for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost_from_start.partial_cmp(&other.cost_from_start)
    }
}
impl Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost_from_start.cmp(&other.cost_from_start)
    }
}