use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: i64,
        CA: [(i64, [i64; M]); N],
    }

    let mut stack = Vec::new();
    //
    stack.push((0, 0, vec![0; M]));

    let mut ans = std::i64::MAX; // 1e5よりでかいので多分 ok
    while let Some((i, mut cost, score)) = stack.pop() {
        println!("pop {}, {}, {:?}", i, cost, score);
        // 終了判定
        if score.iter().all(|&x| x >= X) {
            ans = cmp::min(ans, cost);
            continue;
        }

        if i >= N {
            continue;
        }

        // i + 1 番目の本を購入しない場合
        stack.push((i + 1, cost, score.clone()));

        // i + 1 番目の本を購入する場合
        let (c, A) = &CA[i];
        cost += c;
        let score_new: Vec<i64> = score.iter().zip(A.iter()).map(|(a, b)| a + b).collect();
        stack.push((i + 1, cost, score_new));
    }

    if ans == std::i64::MAX {
        ans = -1;
    }

    println!("{}", ans);
}
