use proconio::{fastout, input};

fn f(x: u32) -> u32 {
    x * x + 2 * x + 3
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        t: u32,
    }

    let ans = f(f(f(t) + t) + f(f(t)));
    println!("{}", ans);
}
