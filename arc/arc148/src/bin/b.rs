use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let k = s.iter().position(|&x| x == 'p');

    let mut ans = s.iter().collect::<String>();
    if k.is_none() {
        println!("{}", ans);
        return;
    }

    let l = k.unwrap();

    for r in l..n {
        let head: String = s[..l].iter().collect();
        let mid: String = s[l..=r]
            .iter()
            .rev()
            .map(|&x| if x == 'p' { 'd' } else { 'p' })
            .collect();
        let tail: String = s[(r + 1)..].iter().collect();
        let tmp = format!("{}{}{}", head, mid, tail);
        if tmp < ans {
            ans = tmp;
        }
    }

    println!("{}", ans);
}
