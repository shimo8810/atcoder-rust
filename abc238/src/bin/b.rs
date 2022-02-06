use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u32; N]
    }

    let mut cut = vec![0];

    let mut c = 0;
    for a in &A {
        c = (c + a) % 360;
        cut.push(c);
    }
    cut.sort_unstable();
    cut.push(360);

    let ans = cut.windows(2).map(|x| x[1] - x[0]).max().unwrap();
    println!("{}", ans);
}
