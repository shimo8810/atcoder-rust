use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut K: u64,
        mut AB: [(u64, u64); N]
    }
    AB.sort_unstable();

    for (a, b) in AB.into_iter() {
        if K < a {
            break;
        }
        K += b;
    }
    println!("{}", K);
}
