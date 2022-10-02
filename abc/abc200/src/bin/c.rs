use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut cnts = vec![0i64; 200];

    for x in a.into_iter() {
        cnts[x % 200] += 1;
    }

    let ans: i64 = cnts.into_iter().map(|x| x * (x - 1) / 2).sum();
    println!("{}", ans);
}
