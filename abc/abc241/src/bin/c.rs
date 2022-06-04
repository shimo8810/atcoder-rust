use proconio::{fastout, input, marker::Chars};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    }

    let mut ans = 0;

    for i in 0..N {
        for j in 0..=(N - 6) {
            if S[i][j..(j + 6)].iter().filter(|&c| *c == '#').count() >= 4 {
                println!("{}", YES);
                return;
            }
            if S[j..(j + 6)].iter().filter(|&t| t[i] == '#').count() >= 4 {
                println!("{}", YES);
                return;
            }
        }
    }

    for i in 0..=(N - 6) {
        for j in 0..=(N - 6) {
            let mut count = 0;
            for k in 0..6 {
                count += if S[i + k][j + k] == '#' { 1 } else { 0 };
            }
            if count >= 4 {
                println!("{}", YES);
                return;
            }

            let mut count = 0;
            for k in 0..6 {
                count += if S[i + k][j + 5 - k] == '#' { 1 } else { 0 };
            }
            if count >= 4 {
                println!("{}", YES);
                return;
            }
        }
    }

    println!("{}", NO);
}
