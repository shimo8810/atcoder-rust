use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: usize,
        B: usize,
        mut W: usize
    }
    // kg -> g
    W *= 1000;

    let mx = W / A;
    let mn = (W + B - 1) / B;

    if mx < mn {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", mn, mx);
    }
}
