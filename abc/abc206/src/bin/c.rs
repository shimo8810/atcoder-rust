use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u64; N]
    }

    let mut map = std::collections::HashMap::new();
    for &a in &A {
        *map.entry(a).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (_, n) in map.into_iter() {
        ans += (N - n) * n;
    }
    println!("{}", ans / 2);
}
