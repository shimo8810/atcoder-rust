use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut P: u32,
    }

    let coin: Vec<_> = (1..=10).scan(1, |prev, x| {
        *prev *= x;
        Some(*prev)
    }).collect();

    let mut ans = 0;
    for &c in coin.iter().rev() {
        ans += P / c;
        P %= c;
    }
    println!("{}", ans);
}
