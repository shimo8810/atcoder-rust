use proconio::{fastout, input, marker::Usize1};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }

    let mut check = vec![vec![0; N]; N];

    for _ in 0..M {
        input! {K: usize}
        input! {X: [Usize1; K]}
        for &x1 in X.iter() {
            for &x2 in X.iter() {
                check[x1][x2] += 1;
                check[x2][x1] += 1;
            }
        }
    }

    let ans = if check.into_iter().flatten().min().unwrap() < 2 {
        NO
    } else {
        YES
    };
    println!("{}", ans);
}
