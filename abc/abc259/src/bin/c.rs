use proconio::{fastout, input, marker::Chars};

const YES: &str = "Yes";
const NO: &str = "No";

fn f(S: &[char]) -> Vec<(char, u64)> {
    let mut ans = vec![];
    let mut prev = S[0];
    let mut i = 0;
    while i < S.len() {
        let mut cnt = 0;
        while i < S.len() && prev == S[i] {
            cnt += 1;
            i += 1;
        }
        ans.push((prev, cnt));

        if i < S.len() {
            prev = S[i];
        }
    }

    ans
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
        T: Chars
    }

    let s = f(&S);
    let t = f(&T);
    if s.len() != t.len() {
        println!("{}", NO);
        return;
    }

    for (&(c1, n1), &(c2, n2)) in s.iter().zip(t.iter()) {
        // 文字の一致
        if c1 != c2 {
            println!("{}", NO);
            return;
        }
        // 1文字のときの一致
        if n1 == 1 && n2 != 1 {
            println!("{}", NO);
            return;
        }

        if n1 > n2 {
            println!("{}", NO);
            return;
        }
    }
    println!("{}", YES);
    return;
}
