use proconio::{fastout, input, marker::Usize1};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! { N: usize }
    let mut list = vec![vec![]; N];

    for _ in 1..N {
        input! {a: Usize1, b: Usize1}
        list[a].push(b);
        list[b].push(a);
    }

    let ans = if list.iter().map(|v| v.len()).max().unwrap() == N - 1 {
        YES
    } else {
        NO
    };

    println!("{}", ans);
}
