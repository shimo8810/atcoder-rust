use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [u64; N],
    }

    let mut v = vec![0; N + 1];
    let mut prev = 0;
    for (i, &a) in A.iter().enumerate() {
        v[i + 1] = a - prev - 1 + v[i];
        prev = a;
    }

    for _ in 0..Q {
        input! {K: u64}
        let mut ok = 0;
        let mut ng = v.len();

        while ng - ok > 1 {
            let md = (ok + ng) / 2;
            if v[md] < K {
                ok = md;
            } else {
                ng = md;
            }
        }
        let ans = if ok == 0 { 0 } else { A[ok - 1] } + K - v[ok];
        println!("{}", ans);
    }
}
