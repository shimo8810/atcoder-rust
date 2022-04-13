use proconio::{fastout, input};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        ST: [(String, String); N]
    }

    for (i, (s1, t1)) in ST.iter().enumerate() {
        let mut flag2 = false;
        for s in &[s1, t1] {
            let mut flag1 = true;
            for (j, (s2, t2)) in ST.iter().enumerate() {
                if i == j {
                    continue;
                }
                if !(*s != s2 && *s != t2) {
                    flag1 = false;
                    break;
                }
            }
            if flag1 {
                flag2 = true;
            }
        }
        // 全部ok ならok
        if !flag2 {
            println!("{}", NO);
            return;
        }
    }
    println!("{}", YES);
}
