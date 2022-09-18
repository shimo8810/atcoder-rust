use std::vec;

use proconio::{fastout, input};
use std::collections::VecDeque;

const YES: &str = "Yes";
const NO: &str = "No";

// 交わっていればtrue
#[allow(non_snake_case)]
fn f(x1: i64, y1: i64, r1: i64, x2: i64, y2: i64, r2: i64) -> bool {
    // 中心感の距離
    let D2 = (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2);
    //
    let R2 = (r1 + r2) * (r1 + r2);
    let R3 = (r1 - r2) * (r1 - r2);

    D2 <= R2 && D2 >= R3
}

fn g(x1: i64, y1: i64, r1: i64, x2: i64, y2: i64) -> bool {
    (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) - r1 * r1 == 0
}
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        (sx, sy, tx, ty): (i64, i64, i64, i64),
        XYR: [(i64, i64, i64); N]
    }

    let mut graph = vec![vec![]; N + 2];
    for i in 0..N {
        for j in (i + 1)..N {
            let (x1, y1, r1) = XYR[i];
            let (x2, y2, r2) = XYR[j];

            if f(x1, y1, r1, x2, y2, r2) {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }
    for i in 0..N {
        let (x1, y1, r1) = XYR[i];
        if g(x1, y1, r1, sx, sy) {
            graph[i].push(N);
            graph[N].push(i);
        }

        if g(x1, y1, r1, tx, ty) {
            graph[i].push(N + 1);
            graph[N + 1].push(i);
        }
    }

    let mut check = vec![false; N + 2];
    let mut que = VecDeque::new();
    que.push_back(N);

    while let Some(u) = que.pop_front() {
        if u == N + 1 {
            println!("{}", YES);
            return;
        }
        if check[u] {
            continue;
        }
        check[u] = true;

        for &v in graph[u].iter() {
            que.push_back(v);
        }
    }
    println!("{}", NO)
}
