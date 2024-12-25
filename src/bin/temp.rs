use nalgebra::{Matrix2, Vector2};
#[allow(non_snake_case)]
fn main() {
        let A = Matrix2::new(94.0, 22.0, 34.0, 67.0);
        let A = A.lu(); // deconstruct matrix: LU with partial pivoting, ei gausian elimination
        let B = Vector2::new(8400.0, 5400.0);
        let x = A.solve(&B);
        println!("{:?}", x);
}