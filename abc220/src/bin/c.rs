use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [u64; N],
        X: u64
    }

    let s: u64 = A.iter().sum();
    let m = (X / s) as usize;
    let r = X % s;
    let i = A
        .iter()
        .scan(0, |s, &x| {
            *s += x;
            Some(*s)
        })
        .position(|x| x > r)
        .unwrap();
    let ans = A.len() * m + i + 1;
    println!("{}", ans);
}
