use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut a = vec![vec![0; N]; N];
    for i in 0..N {
        for j in 0..=i {
            if j == 0 || j == i {
                a[i][j] = 1
            } else {
                a[i][j] = a[i - 1][j - 1] + a[i - 1][j];
            }
        }
        for x in 0..=i {
            print!("{} ", a[i][x]);
        }
        println!();
    }
}
