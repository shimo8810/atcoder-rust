use proconio::{fastout, input};
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [(i64, i64, i64); N]
    }

    let mut ng = 0i64;
    let mut ok = 1i64 << 32;
    while ok - ng > 1 {
        let mid = (ng + ok) / 2;

        let mut graph = vec![vec![]; N];
        for i in 0..N {
            for j in (i + 1)..N {
                let (x1, y1, p1) = A[i];
                let (x2, y2, p2) = A[j];

                if p1 * mid >= (x1 - x2).abs() + (y1 - y2).abs() {
                    graph[i].push(j);
                }
                if p2 * mid >= (x1 - x2).abs() + (y1 - y2).abs() {
                    graph[j].push(i);
                }
            }
        }

        let mut flag = false;
        for s in 0..N {
            let mut set = HashSet::new();
            let mut que = VecDeque::new();
            que.push_back(s);
            while let Some(u) = que.pop_front() {
                if set.contains(&u) {
                    continue;
                }
                set.insert(u);

                for &v in &graph[u] {
                    que.push_back(v);
                }
            }

            if set.len() == N {
                flag = true;
                break;
            }
        }

        if flag {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}
