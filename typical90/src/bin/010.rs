use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! { N: usize }
    let mut c0 = vec![0; N + 1];
    let mut c1 = vec![0; N + 1];
    for i in 1..=N {
        input! { c: u64, p: u64 }
        c0[i] = (2 - c) * p + c0[i - 1];
        c1[i] = (c - 1) * p + c1[i - 1];
    }

    input! { Q: usize }
    for _ in 0..Q {
        input! { l: usize, r: usize }
        println!("{} {}", c0[r] - c0[l - 1], c1[r] - c1[l - 1]);
    }
}
