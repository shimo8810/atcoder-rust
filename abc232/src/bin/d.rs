use proconio::{fastout, input, marker::Chars};
use std::cmp::max;
use std::collections::HashSet;
use std::collections::VecDeque;
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      H: usize,
      W: usize,
      C: [Chars; H]
    }

    let mut q = VecDeque::new();
    q.push_back((0, 0, 1));
    let mut v = HashSet::new();
    v.insert((0, 0));
    let mut ans = 0;
    while let Some((x, y, z)) = q.pop_front() {
        ans = max(ans, z);
        if y + 1 < H && C[y + 1][x] == '.' && !v.contains(&(x, y + 1)) {
            v.insert((x, y + 1));
            q.push_back((x, y + 1, z + 1));
        }
        if x + 1 < W && C[y][x + 1] == '.' && !v.contains(&(x + 1, y)) {
            v.insert((x + 1, y));
            q.push_back((x + 1, y, z + 1));
        }
    }

    println!("{}", ans);
}
