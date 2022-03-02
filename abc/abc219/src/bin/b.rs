use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      S: [String; 3],
      T: Chars
    }

    for &t in T.iter() {
        let t = (t as usize) - '0' as usize;
        print!("{}", S[t - 1]);
    }
    println!();
}
