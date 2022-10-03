use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize
    }
    let mut list = vec![];
    for _ in 0..N {
        input! {
            L: usize,
            A: [u32; L]
        }
        list.push(A);
    }

    for _ in 0..Q {
        input! {s: Usize1, t: Usize1}

        println!("{}", list[s][t]);
    }
}
