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
    queue.push_back((A, 0));

    while let Some(((x, y), k)) = queue.pop_front() {
        if B == (x, y) {
            println!("{}", k);
            return;
        }
        visited[x][y] = true;

        let mut i = 1;
        while x >= i && y >= i && S[x - i][y - i] == '.' {
            if !visited[x - i][y - i] {
                queue.push_back(((x - i, y - i), k + 1));
            }
            i += 1;
        }
        i = 1;
        while x + i < N && y + i < N && S[x + i][y + i] == '.' {
            if !visited[x + i][y + i] {
                queue.push_back(((x + i, y + i), k + 1));
            }
            i += 1;
        }
        i = 1;
        while x >= i && y + i < N && S[x - i][y + i] == '.' {
            if !visited[x - i][y + i] {
                queue.push_back(((x - i, y + i), k + 1));
            }
            i += 1;
        }
        i = 1;
        while x + i < N && y >= i && S[x + i][y - i] == '.' {
            if !visited[x + i][y - i] {
                queue.push_back(((x + i, y - i), k + 1));
            }
            i += 1;
        }
    }
    println!("-1");
    // let mut ans = 0;
    // println!("{}", ans);
}
