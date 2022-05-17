use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: Chars
    }

    let mut v = vec![];

    for i in 1..(N - 1) {
        if S[i - 1..i + 2].iter().collect::<String>() == "ARC" {
            let mut l = i - 1;
            let mut r = i + 1;
            while l > 0 && S[l - 1] == 'A' {
                l -= 1;
            }

            while r + 1 < N && S[r + 1] == 'C' {
                r += 1;
            }
            v.push((i - l).min(r - i));
        }
    }
    let ans = (v.len() * 2).min(v.iter().sum());
    println!("{}", ans);
}
