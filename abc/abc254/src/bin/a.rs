use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }


    let  ans = N  % 100;
    println!("{:>02}", ans);
}
