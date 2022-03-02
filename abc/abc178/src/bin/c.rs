use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize
    }

    let m = 1000_000_000 + 7;
    let mut a: i64 = 1;
    let mut b: i64 = 1;
    let mut c: i64 = 1;

    for _ in 0..N {
        a = (a * 10) % m;
        b = (b * 9) % m;
        c = (c * 8) % m;
    }

    let ans = ((a + c + m) - 2 * b + m) % m;
    println!("{}", ans);
}
