use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        M: usize,
        T: [(Usize1, Usize1); M]
    }

    let mut count_h = vec![0; H];
    let mut count_w = vec![0; W];

    for &(y, x) in T.iter() {
        count_h[y] += 1;
        count_w[x] += 1;
    }
    let max_h = count_h.iter().max().unwrap();
    let max_w = count_w.iter().max().unwrap();

    let a = count_h.iter().filter(|y| *y == max_h).count();
    let b = count_w.iter().filter(|x| *x == max_w).count();
    let c = T
        .iter()
        .filter(|(y, x)| count_h[*y] == *max_h && count_w[*x] == *max_w)
        .count();

    println!("{}", max_h + max_w - if a * b == c { 1 } else { 0 });
}
