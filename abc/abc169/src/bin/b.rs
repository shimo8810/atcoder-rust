use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [u128; N],
    }
    for &a in A.iter() {
        if a == 0 {
            println!("0");
            return;
        }
    }
    let mut ans = 1;
    for &a in A.iter() {
        ans *= a;
        if ans > 1e18 as u128 {
            println!("-1");
            return;
        }
    }

    println!("{}", ans);
}
