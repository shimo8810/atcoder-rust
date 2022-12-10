use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
fn f(S: &mut [char]) -> Vec<char> {
    S.sort_unstable();
    let g2: i64 = S.iter().collect::<String>().parse().unwrap();
    let g1: i64 = S.iter().rev().collect::<String>().parse().unwrap();

    (g1 - g2).to_string().chars().collect()
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut N: Chars,
        K: usize
    }

    for _ in 0..K {
        N = f(&mut N);
    }
    let ans: i64 = N.into_iter().collect::<String>().parse().unwrap();
    println!("{}", ans);
}
