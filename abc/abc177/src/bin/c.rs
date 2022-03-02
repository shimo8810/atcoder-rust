use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [u64; N]
    }

    let M: u64 = 1_000_000_007;

    let mut ans = 0;
    let mut cum = 0;
    for a in A.iter().rev() {
        ans = (ans % M + (a * cum) % M) % M;
        cum = (cum % M + a % M) % M;
    }

    println!("{}", ans % M);
}
