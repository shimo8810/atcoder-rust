use proconio::input;
use proconio::marker::Usize1;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        H: [u64; N],
    }
    let mut max = vec![0; N];
    for _ in 0..M {
        input! {
            A: Usize1,
            B: Usize1,
        }

        max[A] = cmp::max(max[A], H[B]);
        max[B] = cmp::max(max[B], H[A]);
    }

    let mut ans = 0;
    for (h, m) in H.iter().zip(max.iter()) {
        if h > m {
            ans += 1;
        }
    }
    println!("{}", ans);
}
