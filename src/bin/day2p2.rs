fn main() {
    // let input = 
    //     "7 6 4 2 1
    // 1 2 7 8 9
    // 9 7 6 2 1
    // 1 3 2 4 5
    // 8 6 4 4 1
    // 1 3 6 7 9";
    let input = include_str!("inputs/day2");
    let reports: Vec<Vec<i32>> = input.lines()
        .map(|line|line
            .split_ascii_whitespace()
            .map(|e| e.parse::<i32>().expect("parse error"))
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    let safecount: usize = reports.into_iter()
    // .map(is_safe)//part 1
    .map(is_safe_with_dampener)//part 2
    .filter(|&x| x==true)
    .count(); 
    println!("{safecount}")
}

fn is_safe_with_dampener(report: Vec<i32>) -> bool {
    if is_safe(&report) {return true;}
    let all_report_with_one_missing = (0..report.len()).map(|i| -> Vec<i32> {
        let mut x: Vec<i32> = report.clone();
        x.remove(i);
        x
    }).collect::<Vec<_>>();
    all_report_with_one_missing.iter().any(is_safe)
}

fn is_safe(report: &Vec<i32>) -> bool{
    if report[0] == report[1] {return false;}// edge case where first two level readings are the same
    let increasing = report[0] < report[1];
    let safe_levels = report.windows(2).map(|e| {
        let lhs = e[0];
        let rhs = e[1];

        // the three constraints
        (lhs - rhs).abs() >= 1 &&
        (lhs - rhs).abs() <= 3 &&
        (lhs < rhs) == increasing
    }).collect::<Vec<_>>();
    safe_levels.into_iter().any(|x| x==false) == false //this will return true if none of the levels are un-safe
}