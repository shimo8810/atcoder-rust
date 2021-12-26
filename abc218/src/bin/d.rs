use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize
    }
    let mut set = HashSet::new();

    for _ in 0..N {
        input! {x: u32, y: u32}
        set.insert((x, y));
    }
    let mut ans = 0;
    for &(x1, y1) in set.iter() {
        // 左上
        for &(x2, y2) in set.iter() {
            // 右下
            if x1 == x2 && y1 == y2 {
                continue;
            }

            println!("{}-{} vs {}-{}", x1, y1, x2, y2);
            if set.contains(&(x1, y2)) && set.contains(&(x2, y1)) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
