use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u32,
        A:[i64; N]
    }

    let ans: i64 = (N as i64) * A.iter().map(|&x| x * x).sum::<i64>()
        - A.iter().sum::<i64>() * A.iter().sum::<i64>();

    println!("{}", ans);
}
