use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [u64; N]
    }

    let mut ans = 0;
    let mut pre = 0;
    for a in A {
        if pre > a {
            ans += pre - a;
        } else {
            pre = a;
        }
    }

    println!("{}", ans);
}
