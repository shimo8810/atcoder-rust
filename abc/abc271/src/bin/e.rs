use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        ABC: [(Usize1, Usize1, u32); N],
        E: [Usize1; M]
    }

    // 有効な1->Nへの経路を求める O(N)

}
