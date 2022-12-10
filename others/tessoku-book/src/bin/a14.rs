use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: u32,
        A: [u32; N],
        B: [u32; N],
        C: [u32; N],
        D: [u32; N],
    }

    // N^2
    let mut P = vec![];
    for &a in &A {
        for &b in &B {
            P.push(a + b);
        }
    }
    // N^2
    let mut Q = vec![];
    for &c in &C {
        for &d in &D {
            Q.push(c + d);
        }
    }

    // N^2 log N
    Q.sort_unstable();

    // N^2 log N
    for &p in &P {
        if Q.binary_search(&(K - p)).is_ok() {
            println!("{}", YES);
            return;
        }
    }

    println!("{}", NO);
}
