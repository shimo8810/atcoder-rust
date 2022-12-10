use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: u64,
        A: [u64; N]
    }
    let mut Z = vec![0; N + 1];
    for i in 0..N {
        Z[i + 1] = Z[i] + A[i];
    }

    let mut ans = 0;
    let mut t = 0;

    println!("{:?}", Z);
    for h in 0..(N - 1) {
        while t < N - 1 && Z[t + 1] - Z[h] <= K {
            // println!("{} - {}", h, t + 1);
            t += 1;
        }
        println!("{} - {}", h, t);
        ans += t - h;
    }

    println!("{}", ans);
}
