use regex::Regex;
use nalgebra::{Matrix2, Vector2};
const FLOAT_ERR_THREASHOLD: f64 = 0.001;
#[allow(non_snake_case)]
fn main() {
    // let input = include_str!("tests/day13");
    let input = include_str!("inputs/day13");
    let claw_machines: Vec<((f64, f64, f64, f64), (f64, f64))> = parse(input);
    let mut total_cost = 0.0;
    for claw_machine in claw_machines {
        let ((Xa, Xb, Ya, Yb),(Tx, Ty)) = claw_machine;
        // Ax = B
        let A = Matrix2::new(Xa, Xb, Ya, Yb);
        let B = Vector2::new(Tx, Ty);
        // x = A^-1 * B
        let x = A.try_inverse().unwrap() * B;
        // if x is not integers, then this claw machine is impossible to win.
        let mut is_possible = true;
        let x_rounded = x.map(|e|{
            if (e - e.round()).abs() < FLOAT_ERR_THREASHOLD {
                is_possible = true;
                // round the floating point error away
                e.round()
            } else {
                is_possible = false;
                e
            }
        });
        if !is_possible {continue;}
        let (a_presses, b_presses): (f64, f64) = (x_rounded.x , x_rounded.y);
        let cost = 3.0*a_presses + b_presses;
        total_cost += cost;
    };
    println!("total cost: {total_cost}")
}
#[allow(non_snake_case)]
fn parse(input: &str) -> Vec<((f64, f64, f64, f64),(f64, f64))> {
    let regex = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\r\nButton B: X\+(\d+), Y\+(\d+)\r\nPrize: X\=(\d+), Y\=(\d+)").unwrap();
    regex.captures_iter(input).map(|x|{
        let [Xa, Ya,Xb , Yb, Tx, Ty]= x.extract().1.map(|x|x.parse().unwrap());
        // part 1
        ((Xa, Xb, Ya, Yb),(Tx, Ty))
        // // part 2 
        // ((Xa, Xb, Ya, Yb),(Tx + 10000000000000.0, Ty + 10000000000000.0))
    }).collect()
}