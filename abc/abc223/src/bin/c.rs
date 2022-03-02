use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB: [(f32, f32); N]
    }
    let mut t: f32 = AB.iter().map(|&(a, b)| a / b / 2.0).sum();

    let mut ans = 0.0;

    for &(a, b) in AB.iter() {
        ans += a.min(b * t);
        t -= t.min(a / b);
    }

    println!("{}", ans);
}
