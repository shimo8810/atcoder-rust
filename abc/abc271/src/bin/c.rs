use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut A: [u32; N]
    }

    A.sort_unstable();

    let mut ok = 0;
    let mut ng = 1_000_000_001;

    while ng - ok > 1 {
        let md = (ok + ng) / 2;

        // 持っている巻数
        let mut aru = 0;

        let mut muda = 0;
        let mut mrk = HashSet::new();
        for &a in A.iter() {
            if a <= md && !mrk.contains(&a) {
                aru += 1;
                mrk.insert(a);
            } else {
                muda += 1;
            }
        }

        aru += muda / 2;
        // println!("aru {}", aru);
        if aru >= md {
            ok = md;
        } else {
            ng = md;
        }
    }

    println!("{}", ok);
}
