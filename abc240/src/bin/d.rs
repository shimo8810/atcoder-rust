use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
    }

    let mut sum = 0;
    let mut vec: Vec<(u32, u32)> = vec![];
    for _ in 0..N {
        input! { a: u32 }

        if vec.is_empty() || vec.last().unwrap().0 != a {
            vec.push((a, 1));
            sum += 1;
        } else {
            let (b, n) = vec.pop().unwrap();
            sum -= n;
            if b != n + 1 {
                sum += n + 1;
                vec.push((b, n + 1));
            }
        }

        println!("{}", sum);
    }
}
