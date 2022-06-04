use proconio::{fastout, input};
use std::collections::HashSet;

fn prime_factorize(mut n: usize) -> Vec<usize> {
    let mut ans = vec![];
    let mut k = 2;
    while k * k <= n {
        if n % k == 0 {
            ans.push(k);
            while n % k == 0 {
                n /= k;
            }
        }
        k += 1;
    }
    if n != 1 {
        ans.push(n);
    }
    ans
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; N]
    }
    let mut list = vec![true; M + 1];
    list[0] = false;
    let mut set = HashSet::new();
    for &a in &A {
        for x in &prime_factorize(a) {
            set.insert(*x);
        }
    }
    let mut v: Vec<_> = set.into_iter().filter(|&x| x <= M).collect();
    v.sort_unstable();

    for &x in &v {
        if list[x] {
            let mut k = x;
            while k <= M {
                list[k] = false;
                k += x;
            }
        }
    }
    let ans: Vec<_> = list
        .into_iter()
        .enumerate()
        .filter(|(_, s)| *s)
        .map(|(i, _)| i)
        .collect();
    println!("{}", ans.len());
    for x in &ans {
        println!("{}", x);
    }
}
