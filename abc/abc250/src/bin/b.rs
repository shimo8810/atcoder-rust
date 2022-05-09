use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (N, A, B): (usize, usize, usize),
    }

    for x in 0..(N * A) {
        for y in 0..(N * B) {
            let c = if (x / A + y / B) % 2 == 0 { '.' } else { '#' };
            print!("{}", c);
        }
        println!();
    }
}
