use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: [u64; N]
    }

    let mut ans = 0;

    for xs in (0..N).combinations(3) {
        let arr = [L[xs[0]], L[xs[1]], L[xs[2]]];

        if arr[0] == arr[1] || arr[1] == arr[2] || arr[2] == arr[0] {
            continue;
        }

        if arr.iter().sum::<u64>() <= 2 * arr.iter().max().unwrap() {
            continue;
        }

        ans += 1;
    }
    
    println!("{}", ans);
}