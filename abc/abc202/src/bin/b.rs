use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        S: Chars,
    }

    let ans: String = S
        .into_iter()
        .rev()
        .map(|x| match x {
            '6' => '9',
            '9' => '6',
            x => x,
        })
        .collect();

    println!("{}", ans);
}
