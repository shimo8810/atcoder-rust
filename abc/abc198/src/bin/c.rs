use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        R: f64,
        X: f64,
        Y: f64
    }

    let D = (X.powi(2) + Y.powi(2)).sqrt();

    println!(
        "{}",
        if D == R {
            1
        } else if D < 2.0 * R {
            2
        } else {
            (D / R).ceil() as i64
        }
    );
}
