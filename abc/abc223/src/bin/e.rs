use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

fn dfs(i: usize, d: usize, x: i64, y: i64, s: &[i64]) -> bool {
    if d > 2 && x >= 0 && y >= 0 {
        return true;
    }

    if x <= 0 || y <= 0 {
        return false;
    }

    let b1 = dfs(i, d + 1, x, y - (s[(i + d) % 3] + x - 1) / x, s);
    let b2 = dfs(i, d + 1, x - (s[(i + d) % 3] + y - 1) / y, y, s);

    b1 || b2
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: i64,
        Y: i64,
        S: [i64; 3]
    }

    let ans = if (0..3).any(|i| dfs(i, 0, X, Y, &S)) {
        YES
    } else {
        NO
    };

    println!("{}", ans);
}
