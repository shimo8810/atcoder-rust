use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB: [(u64, u64); N]
    }
    let mut A: Vec<u64> = vec![0; N];
    let mut B: Vec<u64> = vec![0; N];

    for i in 0..N {
        let (a, b) = AB[i];
        A[i] = a;
        B[i] = b;
    }
    A.sort();
    B.sort();
0
    let (x, y) = if N % 2 == 0 {
        (A[N / 2 - 1] + A[N / 2], B[N / 2 - 1] + B[N / 2])
    } else {
        (A[(N + 1) / 2 - 1], B[(N + 1) / 2 - 1])
    };
    let ans = y - x + 1;
    println!("{}", ans);
}
