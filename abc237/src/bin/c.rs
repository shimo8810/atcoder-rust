use proconio::{fastout, input, marker::Chars};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
    }

    let h = S.iter().take_while(|&&x| x == 'a').count();

    let t = S.iter().rev().take_while(|&&x| x == 'a').count();
    if t < h {
        println!("{}", NO);
        return;
    } else {
        let l = S.len() - (t - h);
        let s1: String = S[..l].iter().collect();
        let s2: String = S[..l].iter().rev().collect();
        if s1 == s2 {
            println!("{}", YES);
        } else {
            println!("{}", NO);
        }
    }
}
