use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      mut A: u64,
      mut B: u64
    }

    let mut C = A + B;
    while C > 0 {
        let a = A % 10;
        let b = B % 10;
        let c = C % 10;
        if c != a + b {
            println!("Hard");
            return;
        }
        dbg!(C);
        A /= 10;
        B /= 10;
        C /= 10;
    }
    println!("Easy");
}
