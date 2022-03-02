use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! { mut N: u64 }

    let mut ans = 0;
    while N > 0 {
        N >>= 1;
        ans += 1;
    }

    println!("{}", ans - 1);
}
