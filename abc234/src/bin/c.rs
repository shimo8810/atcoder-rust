use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut N: usize,
    }

    let mut s = vec![];
    while N > 0 {
        s.push(if N & 1 == 1 { '2' } else { '0' });
        N >>= 1;
    }
    let s: String = s.iter().rev().collect();
    println!("{}", s);
}
