use proconio::input;
// use std::collections::HashSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }

    let S: Vec<_> = S.chars().map(|c| c as usize - '0' as usize).rev().collect();
    let mut tot = 0;
    let mut x = 1;
    let mut cnt = vec![0; 2019];
    let mut ans = 0;
    for i in S {
        cnt[tot] += 1;
        tot += i * x;
        tot %= 2019;
        ans += cnt[tot];
        x = x * x % 2019;
    }

    println!("{:?}", cnt);

    println!("{}", ans);
}
