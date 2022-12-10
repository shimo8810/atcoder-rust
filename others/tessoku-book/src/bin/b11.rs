use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [u32; N],
        Q: usize,
        X: [u32; Q]
    }
    A.sort_unstable();

    for &x in &X {
        let mut ok = -1;
        let mut ng = N as i32;

        while (ng - ok).abs() > 1 {
            let md = ((ok + ng) / 2) as usize;

            if A[md] < x {
                ok = md as i32;
            } else {
                ng = md as i32;
            }
        }

        println!("{}", ok + 1);
    }
}
