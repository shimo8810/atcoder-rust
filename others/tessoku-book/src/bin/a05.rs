use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: i32,
        K: i32
    }

    let mut ans = 0;

    for i in 1..=N {
        for j in 1..=N {
            let k = K - i - j;
            if 1 <= k && k <= N {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
