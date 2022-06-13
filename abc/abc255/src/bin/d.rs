use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [isize; N]
    }

    A.sort_unstable();
    let mut cumsum = vec![0; A.len() + 1];
    for (i, &a) in A.iter().enumerate() {
        cumsum[i + 1] = cumsum[i] + a;
    }

    for _ in 0..Q {
        input! {X: isize}
        let mut ng = -1;
        let mut ok = A.len() as isize;
        while ok - ng > 1 {
            let md = (ok + ng) / 2;
            if X <= A[md as usize] {
                ok = md;
            } else {
                ng = md;
            }
        }

        let ans =
            X * ok - cumsum[ok as usize] + cumsum[N] - cumsum[ok as usize] - X * (N as isize - ok);

        println!("{}", ans);
    }
}
