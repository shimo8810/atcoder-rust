use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [Usize1; K],
        XY: [(f64, f64); N]
    }

    let mut ans = 0.0;

    for (x1, y1) in XY.iter() {
        let mut dmin = std::f64::MAX;
        for &a in &A {
            let (x2, y2) = XY[a];
            let d = ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
            if d < dmin {
                dmin = d;
            }
        }
        if ans < dmin {
            ans = dmin;
        }
    }

    println!("{}", ans);
}
