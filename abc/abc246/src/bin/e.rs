use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: (Usize1, Usize1),
        B: (Usize1, Usize1),
        S: [Chars; N]
    }

    let mut visited = vec![vec![false; N]; N];
    // visited[A.0][A.1] = true;
    let mut queue = VecDeque::new();
    queue.push_back(A);

    while let Some((x, y)) = queue.pop_front() {
        visited[x][y] = true;

        for dx in (-1, 1) {
            for dy in (-1, 1) {
                //
            }
        }
    }
    // let mut ans = 0;
    // println!("{}", ans);
}
