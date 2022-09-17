use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u64,
        K1: u64
    }

    if K1 % 10 == 0 {
        println!("0");
        return;
    }
    let K2: u64 = format!("{}", K1)
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap();

    if K1 > K2 {
        println!("0");
        return;
    }

    let mut ans = 0;
    let mut d = 1;
    loop {
        if K1 * d <= N {
            ans += 1;
        } else {
            break;
        }

        if K1 != K2 {
            if K2 * d <= N {
                ans += 1;
            } else {
                break;
            }
        }

        d *= 10;
    }

    println!("{}", ans);
}
