#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [u64; N],
        Q: u64,
        BC: [[i64; 2]; Q],
    }
    let mut count_a = [0u64; 1e5 as usize];
    let mut sum = 0;
    for a in A {
        count_a[a as usize] += 1;
        sum += a;
    }

    for bc in BC.iter() {
        let b = bc[0];
        let c = bc[1];
        //
        let n = count_a[b as usize];
        count_a[c as usize] += n;
        count_a[b as usize] = 0;

        if (c - b) > 0 {
            sum = sum + (c - b) as u64 * n;
        } else {
            sum = sum - (b - c) as u64 * n;
        }

        println!("{}", sum);
    }
}
