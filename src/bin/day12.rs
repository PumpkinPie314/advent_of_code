use std::collections::VecDeque;

use advent_of_code::grid::Grid;

fn main() {
    // let input = include_str!("inputs/day12");
    let input = include_str!("tests/day12");
    let lables_grid = Grid::from_str(&input);
    let mut areas: Vec<usize> = vec![];
    let mut perimitors: Vec<usize> = vec![];
    let mut regions_grid: Grid<Option<usize>> = Grid::new_from(vec![vec![None; lables_grid.width()]; lables_grid.height()]);
    let mut id = 0;
    for gaurden_plot in lables_grid.coords() {
        if regions_grid.get(gaurden_plot).unwrap().is_some() {continue;}
        // explore region
        areas.push(0);
        perimitors.push(0);
        let region_lable = lables_grid.get(gaurden_plot).unwrap();
        let mut queue = VecDeque::new();
        queue.push_back(gaurden_plot);
        while !queue.is_empty(){
            let current_plot = queue.pop_front().unwrap();
            regions_grid.set_cell(current_plot, Some(id));
            areas[id] += 1;
            for adj in lables_grid.adjacent_cells(current_plot) {
                if queue.contains(&adj) {continue};
                let found_edge = !lables_grid.contains_coords(adj) || lables_grid.get(current_plot) != lables_grid.get(adj);
                if found_edge {
                    // println!("({}, {}) is an edge!", adj.x, adj.y);
                    perimitors[id] += 1;
                } else {
                    if regions_grid.get(adj).unwrap().is_some() {continue;}
                    queue.push_back(adj);
                }
            }
        }
        let display = Grid::from_flat(regions_grid.clone().into_iter().map(|x|{
            if x == Some(id) {*region_lable} else {'.'}
        }), lables_grid.width());
        // for x in queue.clone() {
        //     display.set_cell(x, 'x');
        // }
        println!("{}", Grid::from_flat(display.into_iter(), lables_grid.width()).to_string());
        println!("area: {}\t perimitor: {}", areas[id], perimitors[id]);
        println!("________________________________________________________________");
        id += 1;
    };
    // calc price
    let total: usize = areas.into_iter().zip(perimitors.into_iter()).map(|(a,p)| a*p).sum();
    println!("{}", total)
}