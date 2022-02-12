use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut S: Chars,
        a: Usize1,
        b: Usize1
    }

    S.swap(a, b);
    let S: String = S.iter().collect();
    println!("{}", S);
}
