use proconio::{fastout, input};

fn f(a: i64, b: i64) -> i64 {
    a * a * a + a * a * b + a * b * b + b * b * b
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: i64,
    }

    let mut ans = std::i64::MAX;
    for a in 0..=1_000_001 {
        let mut ng = -1i64;
        let mut ok = 1_000_001;
        while (ok - ng).abs() > 1 {
            let b = (ok + ng) / 2;
            if f(a, b) >= N {
                ok = b;
            } else {
                ng = b;
            }
        }
        ans = ans.min(f(a, ok));
    }

    println!("{}", ans);
}
