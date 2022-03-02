use proconio::input;
use proconio::marker::Usize1;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }

    let mut v = vec![0; N];
    for _ in 0..K {
        input! {
            D: usize,
            A: [Usize1; D],
        }

        for a in A {
            v[a] += 1;
        }
    }

    println!(
        "{:?}",
        v.iter()
            .fold(0, |acc, &x| { acc + if x == 0 { 1 } else { 0 } })
    );
}
