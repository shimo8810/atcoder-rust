const D: i128 = 1000000007;

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(u64, u64, u64); N], // 縦線
        DEF: [(u64, u64, u64); M], // 横線
    }
}
