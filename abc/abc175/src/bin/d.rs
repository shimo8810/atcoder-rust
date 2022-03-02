use proconio::input;
use proconio::marker::Usize1;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: i64,
        P: [Usize1; N],
        C: [i64; N],
    }

    let mut ans = std::i64::MIN;

    for n in 0..N {
        let mut v = n;
        let mut sum_cycle = 0;
        let mut num_cycle = 0;

        loop {
            num_cycle += 1;
            sum_cycle += C[v];
            v = P[v];
            if v == n {
                break;
            }
        }

        let mut sum = 0;
        let mut cnt = 0;
        loop {
            cnt += 1;
            sum += C[v];

            if cnt > K {
                break;
            }

            let score = sum + cmp::max(0, sum_cycle) * (K - cnt) / num_cycle;
            ans = cmp::max(ans, score);

            v = P[v];
            if v == n {
                break;
            }
        }
    }

    println!("{}", ans);
}
