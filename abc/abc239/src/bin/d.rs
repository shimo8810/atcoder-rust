use proconio::{fastout, input};
use std::collections::HashSet;
const YES: &str = "Takahashi";
const NO: &str = "Aoki";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: u32,
        B: u32,
        C: u32,
        D: u32
    }

    let ps = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211,
    ];
    let ps: HashSet<_> = ps.iter().collect();

    for x in A..=B {
        let mut ans = true;
        // すべてのyに対して素数出ないようなxが存在すればYES
        for y in C..=D {
            if ps.contains(&(x + y)) {
                // x + y が素数でないなら
                ans = false;
                break;
            }
        }
        if ans {
            println!("{}", YES);
            return;
        }
    }
    println!("{}", NO);
}
