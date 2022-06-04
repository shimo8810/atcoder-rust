use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut S: Chars
    }

    S.sort_unstable();
    let ans: String = S.into_iter().collect();
    println!("{}", ans);
}
