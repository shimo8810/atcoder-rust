use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: u64,
        AB: [(u64, u64); N]
    }
    let mut cs = vec![0; N + 1];
    for i in 0..N {
        let (a, b) = AB[i];
        cs[i + 1] = cs[i] + a + b;
    }

    let mut ans = std::u64::MAX;
    for i in 0..N {
        ans = ans.min(cs[i + 1] + (X - i as u64 - 1) * AB[i].1);
    }

    println!("{}", ans);
}
