use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut set = std::collections::HashSet::new();

    let mut a = 2;
    while a * a <= N {
        let mut x = a * a;
        while x <= N {
            set.insert(x);
            x *= a;
        }
        a += 1;
    }
    println!("{}", N - set.len());
}
