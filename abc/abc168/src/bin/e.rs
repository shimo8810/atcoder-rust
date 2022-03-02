#![allow(unused_imports)]

use num::integer::gcd;
use proconio::input;
use std::cmp;
use std::cmp::Reverse;
use std::collections::HashMap;

const D: i128 = 1000000007;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        AB: [(i128, i128); N]
    }

    let mut num_zeros = 0;
    let mut map = HashMap::new();
    for &(mut a, mut b) in AB.iter() {
        //
        if a == 0 && b == 0 {
            num_zeros += 1;
            continue;
        }

        let g = gcd(a, b);
        a /= g;
        b /= g;

        if b < 0 {
            a *= -1;
            b *= -1;
        }

        if b == 0 && a < 0 {
            a *= -1;
            b *= -1;
        }

        let mut rot = false;
        if a <= 0 {
            let tmp = a;
            a = b;
            b = -tmp;
            rot = true;
        }
        let count = map.entry((a, b)).or_insert((0, 0));

        if rot {
            count.1 = (count.1 + 1) % D;
        } else {
            count.0 = (count.0 + 1) % D;
        }
    }

    let mut ans = 1;
    for (_, &(l, r)) in map.iter() {
        let mut sum = 1;
        sum += 2i128.pow(l as u32) % D - 1;
        sum += 2i128.pow(r as u32) % D - 1;
        ans *= sum % D;
    }

    ans += num_zeros - 1;
    println!("{}", ans % D);
}
