use glam::IVec2;

fn main() {
    // let input = include_str!("tests/day21");
    let input = include_str!("inputs/day21");
    let door_codes = input.lines().map(|x|x.to_string()).collect::<Vec<_>>();
    let nums= vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['p', '0', 'A'],
    ];
    let dirs = vec![
        vec!['p', '^', 'A'],
        vec!['<', 'v', '>'],
    ];
    let p1 = door_codes.into_iter().map(|line| {
        let num_pass     = line.clone();
        let first_robot  = to_instructions(&nums, num_pass.clone());
        let second_robot = to_instructions(&dirs, first_robot);
        let human_dir    = to_instructions(&dirs, second_robot);
        // let pass_val      = line.chars().filter(|c|*c!='A').collect::<String>().parse::<usize>().unwrap();
        // human_dir.len() * pass_val
        
        let reconstructed_second_robot = from_instructions(&dirs, human_dir);
        let reconstructed_first_robot = from_instructions(&dirs, reconstructed_second_robot);
        let reconstructed_num_pass = from_instructions(&nums, reconstructed_first_robot);
        reconstructed_num_pass
    })
        // .sum::<usize>()
        .collect::<Vec<_>>()
        ;
    println!("{:?}", p1);
    // this works on the test, but input answere is too big :(    
}
fn from_instructions(layout: &Vec<Vec<char>>, input: String) -> String {
    let mut start: Option<IVec2> = None;
    for (y, l) in layout.into_iter().enumerate() {
        for (x, c) in l.into_iter().enumerate() {
            if *c == 'A' {start = Some(IVec2 {x: x as i32, y: y as i32})}
        }
    }
    if start.is_none() {panic!("no A")}
    let mut pos = start.unwrap();
    let mut out = String::new();
    for instruction in input.chars() {
        match instruction {
            '<' => {pos += IVec2::new(-1, 0)}
            '>' => {pos += IVec2::new( 1, 0)}
            '^' => {pos += IVec2::new( 0, -1)}
            'v' => {pos += IVec2::new( 0,1)}
            'A' => {out.push(layout[pos.y as usize][pos.x as usize]);}
            _ => unreachable!()
        }
    }
    out
}


fn to_instructions(layout: &Vec<Vec<char>>, input: String) -> String {
    let mut input = input.chars().collect::<Vec<_>>();
    input.insert(0, 'A');
    let mut out = String::new();
    for i in 0..input.len()-1 {
        out.push_str(get_delta(layout, input[i], input[i+1]).as_str());
    }
    out
}
fn get_delta(layout:&Vec<Vec<char>>, from: char, to: char) -> String {
    let mut from_pos = None;
    let mut to_pos = None;
    let mut avoid_pos = None;
    for (y, l) in layout.into_iter().enumerate() {
        for (x, c) in l.into_iter().enumerate() {
            if *c == from {from_pos  = Some(IVec2 {x: x as i32, y: y as i32})}
            if *c == to   {to_pos    = Some(IVec2 {x: x as i32, y: y as i32})}
            if *c == 'p'   {avoid_pos= Some(IVec2 {x: x as i32, y: y as i32})}
        }
    }
    if from_pos == None || to_pos == None {panic!("{} or {} are not on the layout!", from, to)}
    let (from_pos, to_pos) = (from_pos.unwrap(), to_pos.unwrap());
    let avoid_pos = avoid_pos.unwrap();

    let delta = to_pos - from_pos;
    // walk horizontaly, and decide if we should do x or y first, in order to avoid avoid_pos
    let mut x = from_pos.x;
    let mut x_first = true;
    for _ in 0..delta.abs().x {
        if !x_first {break}
        x += if delta.x.is_positive() {1} else {-1} ;
        if (IVec2{x, y: from_pos.y} == avoid_pos) {x_first = false;}
    }
    // now we know which to do first
    let mut out = String::new();
    if x_first {
        for _ in 0..delta.abs().x {
            if delta.x.is_positive() {out.push('>');}
            if delta.x.is_negative() {out.push('<');}
        }
        for _ in 0..delta.abs().y {
            if delta.y.is_positive() {out.push('v');}
            if delta.y.is_negative() {out.push('^');}
        }
    } else {
        for _ in 0..delta.abs().y {
            if delta.y.is_positive() {out.push('v');}
            if delta.y.is_negative() {out.push('^');}
        }
        for _ in 0..delta.abs().x {
            if delta.x.is_positive() {out.push('>');}
            if delta.x.is_negative() {out.push('<');}
        }
    }
    out.push('A');
    out
}