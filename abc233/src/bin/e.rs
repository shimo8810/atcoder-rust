use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        X: Chars,
    }

    let xs: Vec<_> = X.iter().map(|&x| (x as u8 - b'0') as u64).collect();

    let mut s: u64 = xs.iter().sum();
    let mut c = 0;
    let mut ans = vec![];
    let mut i = xs.len() as i64 - 1;
    loop {
        c += s;
        ans.push(((c % 10) as u8 + b'0') as char);
        c /= 10;
        if i >= 0 {
            s -= xs[i as usize];
        }

        if i <= 0 && c == 0 {
            break;
        }
        i -= 1;
    }

    let ans: String = ans.iter().rev().collect();
    println!("{}", ans);

    // let a = b'0';
}
