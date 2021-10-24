use proconio::input;
use proconio::marker::Usize1;
use std::cmp;
use std::collections::BTreeMap;

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize
    }

    let mut map = BTreeMap::new();

    let mut C = vec![0; N];
    let mut R = vec![0; N];
    let mut A = vec![0; N];
    for i in 0..N {
        input! {r: Usize1, c: Usize1, a:u64}
        C[i] = c;
        R[i] = r;
        A[i] = a;
        map.entry(a).or_insert_with(Vec::new).push(i);
    }
    let mut dp = vec![0; N];
    let mut cmax = vec![0; W];
    let mut rmax = vec![0; H];
    for (_, list) in map.iter().rev() {
        for &i in list.iter() {
            dp[i] = cmp::max(rmax[R[i]], cmax[C[i]]);
        }
        for &i in list.iter() {
            rmax[R[i]] = cmp::max(rmax[R[i]], dp[i] + 1);
            cmax[C[i]] = cmp::max(cmax[C[i]], dp[i] + 1);
        }
    }

    for i in dp.iter() {
        println!("{}", i);
    }
}
