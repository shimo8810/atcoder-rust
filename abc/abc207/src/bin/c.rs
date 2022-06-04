use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }
    let mut lr = vec![];
    for _ in 0..N {
        input! {t: Usize1, l: u32, r: u32}
        lr.push((
            l * 2 + if t & 2 == 2 { 1 } else { 0 },
            r * 2 - if t & 1 == 1 { 1 } else { 0 },
        ));
    }
    let mut ans = 0;
    for (i, &(l1, r1)) in lr.iter().enumerate() {
        for &(l2, r2) in lr.iter().skip(i + 1) {
            if l1.max(l2) <= r1.min(r2) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
