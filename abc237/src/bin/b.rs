use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[u32; W]; H]
    }

    for i in 0..W {
        for a in &A {
            print!("{} ", a[i]);
        }
        println!();
    }
}
