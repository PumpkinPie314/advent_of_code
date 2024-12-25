use advent_of_code::grid::Grid;
// use glam::Vec2;

fn main() {
    // let grid = Grid::from_str(include_str!("tests/day10"));
    let grid = Grid::from_str(include_str!("inputs/day10"));
    let width = grid.width();
    let grid = Grid::from_flat(grid.into_iter().map(|c|c.to_digit(10).unwrap()), width);
    let mut p1 = 0;
    for (pos, _) in grid.cells_enumerate().filter(|(_, ch)| **ch == 0) {
        let mut display = Grid::new_from(vec![vec!['.'; grid.width()];grid.height()]);
        let mut queue = vec![pos];
        let mut visited = vec![];
        let mut peek_count = 0;
        while !queue.is_empty() {
        // for _ in 0..5 {
            let current_cell = queue.pop().unwrap();
            if *grid.get(current_cell).unwrap() == 9 {peek_count += 1;continue;}
            let current_value = grid.get(current_cell).unwrap();
            visited.push(current_cell);
            display.set_cell(current_cell, '#');
            for adj_pos in grid.adjacent_cells(current_cell) {
                if let Some(adj_value) = grid.get(adj_pos){
                    if *adj_value != current_value + 1 {continue;}
                    if visited.contains(&adj_pos) {continue;} 
                    // un comment below line for p1
                    // if queue.contains(&adj_pos) {continue;} 
                    queue.insert(0, adj_pos);
                }
            }
        }
        p1 += peek_count;
        println!("\n{}", display.clone().to_string())
    }
    println!("p1: {}", p1)
}