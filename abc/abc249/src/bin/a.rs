use proconio::{fastout, input};

fn dist(a: u32, b: u32, c: u32, x: u32) -> u32 {
    (x / (a + c) * a + a.min(x % (a + c))) * b
}
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: u32,
        B: u32,
        C: u32,
        D: u32,
        E: u32,
        F: u32,
        X: u32
    }

    let ans = match dist(A, B, C, X).cmp(&dist(D, E, F, X)) {
        std::cmp::Ordering::Greater => "Takahashi",
        std::cmp::Ordering::Less => "Aoki",
        std::cmp::Ordering::Equal => "Draw",
    };
    println!("{}", ans);
}
