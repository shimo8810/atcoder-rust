use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u32,
    }

    let ans: f64 = (1..N).map(|x| (N as f64) / (x as f64)).sum();
    println!("{}", ans);
}
