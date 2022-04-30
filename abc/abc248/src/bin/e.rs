use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        XY: [(i32, i32); N]
    }

    if K == 1 {
        println!("Infinity");
        return;
    }
    let mut ans = 0;
    let mut check = vec![vec![false; N]; N];
    for i in 0..N {
        check[i][i] = true;
    }
    for (i, (x1, y1)) in XY.iter().enumerate() {
        for (j, (x2, y2)) in XY.iter().enumerate() {
            if check[i][j] {
                continue;
            }
            let mut count = 2;
            let mut vec = vec![i, j];
            for (k, (x3, y3)) in XY.iter().enumerate() {
                if (y3 - y1) * (x2 - x1) == (y2 - y1) * (x3 - x1) {
                    count += 1;
                    vec.push(k);
                }
            }
            for &p in &vec {
                for &q in &vec {
                    check[p][q] = true;
                    check[q][p] = true;
                }
            }
            if count >= K {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
