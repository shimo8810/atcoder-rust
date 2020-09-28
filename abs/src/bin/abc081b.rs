use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        mut a: [u64; n],
    }

    let mut ans = 0;
    'outer: loop {
        for ai in a.iter_mut() {
            //
            if *ai % 2 == 1 {
                break 'outer;
            }
            *ai /= 2;
        }
        ans += 1;
    }

    println!("{}", ans);
}
