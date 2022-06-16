use proconio::{fastout, input, marker::Chars};

fn dfs(pw: &mut Vec<u8>, s: &[char]) -> usize {
    if pw.len() == 4 {
        let mut f = true;
        for (i, &c) in s.iter().enumerate() {
            if c == 'o' {
                f &= pw.iter().any(|&x| x == i as u8);
            } else if c == 'x' {
                f &= pw.iter().all(|&x| x != i as u8);
            }
        }
        return if f { 1 } else { 0 };
    }

    let mut count = 0;
    for x in 0..10 {
        pw.push(x);
        count += dfs(pw, s);
        pw.pop();
    }

    count
}
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
    }

    let mut pw = vec![];
    let ans = dfs(&mut pw, &S);
    println!("{:?}", ans);
}
