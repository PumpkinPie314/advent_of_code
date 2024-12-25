fn main() {
    // let input = 
    //     "7 6 4 2 1
    // 1 2 7 8 9
    // 9 7 6 2 1
    // 1 3 2 4 5
    // 8 6 4 4 1
    // 1 3 6 7 9";
    let input = include_str!("inputs/day2");
    let safecount = input.lines().map(|line| -> bool{
        let reports= line.split_ascii_whitespace().map(|e| e.parse::<i32>().expect("parse error")).collect::<Vec<_>>();
        if reports[0] == reports[1] {return false;}
        let increasing = reports[0] < reports[1];
        for i in 0..=reports.len()-2 {
            if reports[i] == reports[i+1] {return false;}
            if (reports[i] - reports[i+1]).abs() > 3 {return false;}
            if (reports[i] < reports[i+1]) != increasing {return false;}
        };
        true
    }).filter(|x|*x).count();
    println!("{safecount}")
}
