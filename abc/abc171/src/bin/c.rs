#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        mut N: usize,
    }

    let offset = 'a' as u8;
    let mut ans = "".to_string();

    loop {
        if N == 0 {
            break;
        }

        let r = N % 26;
        let c = ((r as u8 + 25) % 26 + offset) as char;

        ans = c.to_string() + &ans;
        if N % 26 == 0 {
            N = N / 26 - 1;
        } else {
            N /= 26;
        }
    }
    println!("{}", ans);
}
