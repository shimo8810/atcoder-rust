use proconio::{fastout, input};

const M: u128 = 998244353;
const DIV2: u128 = 499122177;

fn sum_to(x: u128) -> u128 {
    (((((x + 1) % M) * (x % M)) % M) * DIV2) % M
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u128,
    }

    let mut ans = 0;
    let mut k = 1;
    for _ in 1..=18 {
        k *= 10;
        if k / 10 <= N.min(k - 1) {
            ans = (ans + sum_to(N.min(k - 1) - k / 10 + 1)) % M;
        }
    }

    println!("{}", ans);
}
