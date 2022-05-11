use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut V: i32,
        ABC: [i32; 3]
    }

    loop {
        for (&x, &ans) in ABC.iter().zip(['F', 'M', 'T'].iter()) {
            if V < x {
                println!("{}", ans);
                return;
            } else {
                V -= x;
            }
        }
    }
}
