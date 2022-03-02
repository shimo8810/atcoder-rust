use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      X: u32,
      Y: u32
    }

    if X >= Y {
        println!("0");
    } else {
        let z = Y - X;
        let mut a = z / 10;
        let b = z % 10;
        if b > 0 {
            a += 1;
        }

        println!("{}", a);
    }
}
