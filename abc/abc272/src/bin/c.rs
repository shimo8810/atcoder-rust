use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [i32; N]
    }

    let mut os: Vec<_> = A.iter().filter(|&&x| x % 2 == 1).collect();
    let mut es: Vec<_> = A.iter().filter(|&&x| x % 2 == 0).collect();

    os.sort_unstable();
    es.sort_unstable();

    let o = if os.len() >= 2 {
        **os.iter().rev().next().unwrap() + **os.iter().rev().nth(1).unwrap()
    } else {
        -1
    };

    let e = if es.len() >= 2 {
        **es.iter().rev().next().unwrap() + **es.iter().rev().nth(1).unwrap()
    } else {
        -1
    };

    let ans = o.max(e);

    println!("{}", ans);
}
