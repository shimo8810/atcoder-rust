use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      mut N: u128, // 8 ^ 20 < 18 * 10^18(u64)
      K: usize
    }
    let mut v = Vec::new();

    if N == 0 {
        println!("0");
        return;
    }

    while N > 0 {
        v.push((N % 10) as u8);
        N /= 10;
    }

    for _ in 0..K {
        let mut n = v
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &x)| acc + x as u64 * 8u64.pow(i as u32));
        v.clear();
        while n > 0 {
            let z = match n % 9 {
                8 => 5u8,
                _ => (n % 9) as u8,
            };
            v.push(z);
            n /= 9;
        }
    }

    let ans: String = v.iter().rev().map(|&x| (x + b'0') as char).collect();
    println!("{}", ans);
}
