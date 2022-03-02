use proconio::{fastout, input};

fn f(n: usize, v: &mut Vec<(usize, usize)>, u: &mut [bool], a: &[Vec<u64>]) -> u64 {
    let mut ret = 0;
    if let Some(p) = (0..n).find(|&x| !u[x]) {
        u[p] = true;
        for q in 0..n {
            if !u[q] {
                u[q] = true;
                v.push((p, q));
                ret = ret.max(f(n, v, u, a));
                v.pop();
                u[q] = false;
            }
        }
        u[p] = false;
    } else {
        ret = v.iter().fold(0, |acc, &(i, j)| acc ^ a[i][j]);
    }

    ret
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! { N: usize }
    let mut A = vec![vec![0; 2 * N]; 2 * N];
    for i in 0..(2 * N - 1) {
        for j in (i + 1)..(2 * N) {
            input! { a: u64}
            A[i][j] = a;
            A[j][i] = a;
        }
    }

    let mut vec = vec![];
    let mut used = vec![false; 2 * N];
    let ans = f(2 * N, &mut vec, &mut used, &A);
    println!("{}", ans);
}
