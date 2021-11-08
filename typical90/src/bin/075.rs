use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      mut N: usize,
    }

    let mut n = 0;

    for i in 2.. {
        if i * i > N {
            break;
        }
        while N % i == 0 {
            N /= i;
            n += 1;
        }
    }
    if N != 1 {
        n += 1;
    }

    let ans = (0..).find(|&x| (1 << x) >= n).unwrap();
    println!("{}", ans);
}
