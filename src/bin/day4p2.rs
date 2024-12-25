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
    let mut windows: Vec<[[char; 3]; 3]> = vec![];
    for y in 0..grid.len()-2 {
        for x in 0..grid.len()-2 {
            windows.push([
                [grid[x][y],    grid[x+1][y],   grid[x+2][y]],
                [grid[x][y+1],  grid[x+1][y+1], grid[x+2][y+1]],
                [grid[x][y+2],  grid[x+1][y+2], grid[x+2][y+2]],
            ])
        }
    }
    let mut count = 0;
    for w in windows {
        if w[1][1] != 'A' {continue;}
        let (tl, tr, br, rl) = (w[0][0], w[2][0], w[2][2], w[0][2]);
        match (tl, tr, br, rl) {
            ('M', 'M', 'S', 'S') |
            ('S', 'M', 'M', 'S') |
            ('S', 'S', 'M', 'M') |
            ('M', 'S', 'S', 'M') => {},
            _ => {continue;}
        }

        // println!("{:?}", w[0]);
        // println!("{:?}", w[1]);
        // println!("{:?}", w[2]);
        // println!("");
        count += 1;
    }
    println!("{}", count);

}