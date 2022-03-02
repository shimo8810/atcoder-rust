use proconio::{fastout, input, marker::Chars};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: Chars,
      T: Chars
    }
    let S: Vec<_> = S.iter().map(|&x| x as i32).collect();
    let T: Vec<_> = T.iter().map(|&x| x as i32).collect();

    let d = (S[0] - T[0] + 26) % 26;

    let ans = if S
        .iter()
        .zip(T.iter())
        .all(|(&x, &y)| (y + d) % 26 == x % 26)
    {
        YES
    } else {
        NO
    };

    println!("{}", ans);
}
