use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      Q: usize,
      mut A: [u32; N]
    }

    A.sort_unstable();

    for _ in 0..Q {
        input! {x: u32}
        // let  = -1;
        // let ok
        let mut ok = -1;
        let mut ng = A.len() as i64;
        while (ok - ng).abs() > 1 {
            let m = (ok + ng) / 2;
            if A[m as usize] < x {
                ok = m;
            } else {
                ng = m;
            }
        }
        println!("{} ", A.len() - ng as usize);
    }
}
