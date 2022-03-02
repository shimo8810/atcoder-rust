use proconio::input;
use proconio::marker::Chars;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
        T: Chars,
    }

    let mut ans = 1001;
    let sl = S.len();
    let tl = T.len();
    for i in 0..=(sl - tl) {
        let mut cnt = tl;
        for j in 0..tl {
            if S[i + j] == T[j] {
                cnt -= 1;
            }
        }
        ans = cmp::min(ans, cnt);
    }

    println!("{}", ans);
}
