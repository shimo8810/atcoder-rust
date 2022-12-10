use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
    }

    let mut ans = -1;
    for (i, c) in S.into_iter().enumerate() {
        if c == 'a' {
            ans = i as i32 + 1;
        }
    }

    println!("{}", ans);
}
