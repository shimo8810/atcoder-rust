use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: u64,
    }

    let mut ans = 0;

    for i in 2..=N {
        if N == 1 {
            break;
        }

        if i * i > N {
            if N != 1 {
                ans += 1;
            }
            break;
        }

        if N % i == 0 {
            //
            let mut c = 0;
            while N % i == 0 {
                N /= i;
                c += 1;
            }
            ans += ((c as f64 * 2.0 + 0.25).sqrt() - 0.5) as u64;
        }
    }

    println!("{}", ans);
}
