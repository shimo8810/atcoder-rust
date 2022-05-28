use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut K: u64,
        A: [u32; N]
    }

    let p = K / N as u64;
    let q = K as usize % N;

    let mut list: Vec<_> = A.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
    list.sort_unstable();
    let mut v = vec![0; N];
    for (_, i) in list.into_iter().take(q) {
        v[i] += 1;
    }

    for x in v.into_iter() {
        println!("{}", p + x);
    }
}
