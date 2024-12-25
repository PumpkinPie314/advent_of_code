fn main() {
//     let input = 
// "MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX";
    let input = include_str!("inputs/day4");
    let grid: Vec<Vec<char>> = input.lines().map(|l|l.chars().collect()).collect();
    let x_positions: Vec<(usize, usize)> = (0..grid.len()).flat_map(|x|(0..grid[0].len()).map(move |y| (x,y)))
        .filter(|&pos|char_at(&grid, pos) == Some('X'))
        .collect();
    let a_positions: Vec<(usize, usize)> = (0..grid.len()).flat_map(|x|(0..grid[0].len()).map(move |y| (x,y)))
        .filter(|&pos|char_at(&grid, pos) == Some('A'))
        .collect();
    let directions: [(isize, isize); 8] = [
        (-1,-1),( 0,-1),( 1,-1),
        (-1, 0),        ( 1, 0),
        (-1, 1),( 0, 1),( 1, 1),
    ];

    let mut part1_count = 0;
    for x in x_positions.into_iter() {
        for direction in directions.into_iter() {
            if has_mas(&grid, x, direction) {
                part1_count += 1;
            }
        }
    }
    println!("part one count: {part1_count}");

    let mut part2_count = 0;
    for a in a_positions {
        if is_xmas_center(&grid, a) {
            part2_count += 1;
        }
    }
    println!("part two count: {part2_count}");
}
fn has_mas(grid: &Vec<Vec<char>>, start: (usize,usize), direction: (isize, isize)) -> bool{
    let mut pointer_position = (start.0 as isize, start.1 as isize);
    for c in ['M', 'A', 'S'] {
        pointer_position = offset(pointer_position, direction);
        if char_at(grid, pointer_position) != Some(c) {return false;};
    };
    true
}


fn is_xmas_center(grid: &Vec<Vec<char>>, pos: (usize,usize)) -> bool{
    let pos = (pos.0 as isize, pos.1 as isize);
    let cross_offsets = [(-1,-1),( 1,-1),(-1, 1),( 1, 1)];

    let cross_chars: Vec<Option<char>> = cross_offsets.into_iter().map(|delta| char_at(grid, offset(pos, delta))).collect::<Vec<_>>();
    // if any cross chars are hanging off the screen, then they could never be an X-Mas!
    if cross_chars.iter().any(|&x|x.is_none()) {return false;}
    let cross_chars: [char; 4] = cross_chars.into_iter().map(|x|x.unwrap()).collect::<Vec<_>>().try_into().unwrap();

    let (tl, tr, br, rl) = cross_chars.try_into().unwrap();
    match (tl, tr, br, rl) {
        ('M', 'M', 'S', 'S') |
        ('S', 'M', 'M', 'S') |
        ('S', 'S', 'M', 'M') |
        ('M', 'S', 'S', 'M') => {return true;},
        _ => {return false}
    }
}

fn offset(pos:(isize, isize), direction: (isize, isize)) -> (isize, isize){
    (
        pos.0 + direction.0,
        pos.1 + direction.1
    )
} 


fn char_at<T>(grid: &Vec<Vec<char>>, pos: (T, T)) -> Option<char>// this way I can use either usize or isize for pos!
where 
    T: TryInto<isize>
{
    let x: isize = pos.0.try_into().ok()?;
    let y: isize = pos.1.try_into().ok()?;
    if x.is_negative() || y.is_negative() {return None;}
    Some(*grid.get(y as usize)?.get(x as usize)?)
}
