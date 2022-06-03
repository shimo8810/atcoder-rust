use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      mut XY: [(u32, u32); N]
    }

    XY.sort_unstable();

    let mut ans = 0;
    for &(x1, y1) in XY.iter() {
        for &(x2, y2) in XY.iter() {
            if x1 < x2
                && y1 < y2
                && XY.binary_search(&(x1, y2)).is_ok()
                && XY.binary_search(&(x2, y1)).is_ok()
            {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
