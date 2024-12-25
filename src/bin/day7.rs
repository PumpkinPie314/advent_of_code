// 3769341065216
// 3245122495150

fn main() {
    // let input = include_str!("tests/day7");
    let input = include_str!("inputs/day7");
    let input: Vec<(i64, Vec<i64>)>= input.lines().map(|l|{
        let mut sections = l.split(":");
        let target = sections.next().unwrap().parse::<i64>().unwrap();
        let operands = sections.next().unwrap().split_whitespace().map(|n|n.parse::<i64>().unwrap()).collect::<Vec<_>>();
        assert!(sections.next().is_none());
        (target,operands)
    }).collect();
    // for e in &input {
    //     println!("{e:?}")
    // }
    let p1 = input.clone().into_iter()
        .filter(|(target, operands)| {is_reachable(*target,&operands)})
        .map(|(target, _)|target)
        .sum::<i64>();
    println!("{p1:?}")
}
fn is_reachable(target: i64, operands: &Vec<i64>) -> bool{
    if operands.len() == 1 {
        return target == operands[0]
    };
    let mut new_operands = operands.clone();
    let last_operand = new_operands.pop().unwrap();
    if target > last_operand && is_reachable(target-last_operand, &new_operands) {
        return true
    }
    if target % last_operand == 0 && is_reachable(target/last_operand, &new_operands) {
        return true
    }
    if target % last_operand == 0 && is_reachable(target/last_operand, &new_operands) {
        return true
    }
    false
}