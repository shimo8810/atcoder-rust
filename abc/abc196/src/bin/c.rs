use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u64,
    }

    let NN = 1_000_000;

    for i in 1..=NN {
        let ans: u64 = format!("{}{}", i, i).parse().unwrap();
        if ans > N {
            println!("{}", i - 1);
            break;
        }
    }
}
