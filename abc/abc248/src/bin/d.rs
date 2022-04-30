use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize
    }

    let mut vec = vec![vec![]; N + 1];
    for (i, &a) in A.iter().enumerate() {
        vec[a].push(i + 1);
    }

    for _ in 0..Q {
        input! {L: usize, R: usize, X: usize}
        let tmp = &vec[X];
        let l = tmp.binary_search(&L).unwrap_or_else(|x| x);
        let r = tmp.binary_search(&(R + 1)).unwrap_or_else(|x| x);
        println!("{}", r - l);
    }
}
