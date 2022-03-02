use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
    }

    if (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) > 20 {
        println!("{}", NO);
    } else {
        let minx = x1.min(x2);
        let maxx = x1.max(x2);
        let miny = y1.min(y2);
        let maxy = y1.max(y2);

        for x in (minx - 6)..=(maxx + 6) {
            for y in (miny - 6)..=(maxy + 6) {
                if (x1 - x) * (x1 - x) + (y1 - y) * (y1 - y) == 5
                    && (x2 - x) * (x2 - x) + (y2 - y) * (y2 - y) == 5
                {
                    println!("{}", YES);
                    return;
                }
            }
        }
        println!("{}", NO);
    };
}
