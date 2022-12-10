use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [i32; N]
    }

    let mut cs = vec![0; N + 1];
    for i in 0..N {
        cs[i + 1] = cs[i] + A[i];
    }
    for _ in 0..Q {
        input! {L: usize, R: usize}
        let ans = cs[R] - cs[L - 1];
        println!("{}", ans);
    }
}
