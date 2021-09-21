use proconio::input;
use std::collections::HashSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: [[u8;4];4]
    }

    let mut ans = 0;
    let n: u8 = A.iter().flatten().sum();
    for bit in 0..(1 << 16usize) {
        let p: Vec<_> = (0..16usize)
            .filter(|x| bit & (1 << x) != 0)
            .map(|x| (x / 4, x % 4))
            .collect();

        if p.len() == 0 {
            continue;
        }

        // チェック
        let s: HashSet<_> = [p[0]].iter().collect();
        loop {
            
            for i in p.iter() {

            }
        }

        let m = p.iter().filter(|(x, y)| A[*y][*x] == 1).count() as u8;
        if m == n {
            ans += 1;
        }
    }
    println!("{}", ans);
}
