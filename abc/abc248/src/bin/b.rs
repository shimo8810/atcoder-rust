use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: i64,
        B: i64,
        K: i64
    }

    let mut ans = 0;
    while A * K.pow(ans) < B {
        ans += 1;
    }
    println!("{}", ans);
}

// A * K ^ n >= B を満たす最小のm
