use proconio::{fastout, input};
use std::cmp::Ordering;

struct Ratio {
    n: u32,
    d: u32,
}
impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.n * other.d).cmp(&(self.d * other.n)))
    }
}

impl PartialEq for Ratio {
    fn eq(&self, other: &Self) -> bool {
        self.n * other.d == self.d * other.n
    }
}

impl Ratio {
    fn new(n: u32, d: u32) -> Self {
        Self { n, d }
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
    }

    let a = Ratio::new(2, 0);
    let b = Ratio::new(1, 0);
}
