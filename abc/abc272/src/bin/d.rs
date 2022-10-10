use proconio::{fastout, input};
use std::collections::HashMap;
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: i32
    }

    let mut x = 0;
    let mut sq = HashMap::new();
    while x <= M {
        sq.insert(x * x, x);
        x += 1;
    }

    // グラフ作成
    let mut graph = vec![vec![]; N * N];

    for i in 0..N {
        for j in 0..N {
            let idx = i * N + j;
            // 縦を固定して横が満たすか判定する
            for k in 0..N {
                // l = j +- sqrt(M - (k - i)^2)
                let k = k as i32;
                let i = i as i32;
                let j = j as i32;
                let d2 = M - (k - i) * (k - i);

                if let Some(&d) = sq.get(&d2) {
                    let l = j + d;

                    if 0 <= l && l < N as i32 {
                        let kdx = k as usize * N + l as usize;
                        graph[idx].push(kdx);
                    }

                    if d == 0 {
                        continue;
                    }
                    let l = j - d;
                    if 0 <= l && l < N as i32 {
                        let kdx = k as usize * N + l as usize;
                        graph[idx].push(kdx);
                    }
                }
            }
        }
    }

    let mut ans = vec![vec![std::i32::MAX; N]; N];
    ans[0][0] = 0;
    let mut que = VecDeque::new();

    que.push_back((0, 0, 0));

    while let Some((i, j, s)) = que.pop_front() {
        let idx = i * N + j;
        for kdx in graph[idx].iter() {
            let k = kdx / N;
            let l = kdx % N;
            if ans[k][l] == std::i32::MAX {
                ans[k][l] = s + 1;
                que.push_back((k, l, s + 1));
            }
        }
    }

    // 解答出力
    if *ans.iter().flatten().max().unwrap() == std::i32::MAX {
        println!("-1");
    } else {
        for row in ans.iter() {
            for x in row.iter() {
                if *x == std::i32::MAX {
                    print!("_ ");
                } else {
                    print!("{} ", x);
                }
            }
            println!();
        }
    }
}
