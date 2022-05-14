use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        _W: usize,
    }

    let mut ans = vec![];
    for i in 1..100 {
        ans.push(i);
        ans.push(i * 100);
        ans.push(i * 10000);
    }
    println!("{}", ans.len());
    for i in &ans {
        print!("{} ", i);
    }
}
