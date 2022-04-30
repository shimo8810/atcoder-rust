
use proconio::{fastout, input};

fn divisors(x: usize) -> Vec<usize> {
    let mut a = 1;
    let mut v = vec![];
    while a * a <= x {
        if x % a == 0 {
            v.push(a);
            if x / a != a {
                v.push(x / a);
            }
        }
        a += 1;
    }

    v
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut count = vec![0usize; A.iter().max().unwrap() + 1];
    for &a in &A {
        count[a] += 1;
    }
    let mut ans = 0;
    for &a in &A {
        for &b in &divisors(a) {
            let c = a / b;
            ans += count[b] * count[c];
        }

    }

    println!("{}", ans);
}
