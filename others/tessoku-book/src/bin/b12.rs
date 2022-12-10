use proconio::{fastout, input};

fn f(x: f64) -> f64 {
    x * x * x + x
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: f64,
    }

    let mut h = 0.0;
    let mut t = 100.0;

    while (f(t) - N).abs() > 0.001 {

        let m = (h + t) / 2.0;
        let v = f(m);

        if v >= N {
            t = m;
        } else {
            h = m;
        }
    }

    println!("{}", t);
}
