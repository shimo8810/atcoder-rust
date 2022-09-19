use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        S: Chars
    }

    let mut k = 0;
    for _ in 0..Q {
        input! {t: u8, x: usize}

        if t == 1 {
            k += x;
        } else {
            println!("{}", S[(N + x - k % N - 1) % N]);
        }
    }
}
