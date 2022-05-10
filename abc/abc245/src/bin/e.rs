use proconio::{fastout, input};
use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        (N, M): (usize, usize),
        A: [u32; N],
        B: [u32; N],
        C: [u32; M],
        D: [u32; M]
    }
    let mut chocos: Vec<_> = A.iter().zip(B.iter()).map(Reverse).collect();
    let mut boxes: Vec<_> = C.iter().zip(D.iter()).map(Reverse).collect();
    chocos.sort_unstable();
    boxes.sort_unstable();

    let mut bi = 0;
    let mut set = BTreeSet::new();
    let mut map = HashMap::new();

    for &Reverse((&cx, &cy)) in &chocos {
        while bi < M && cx <= (*boxes[bi].0 .0) {
            // let bx = *boxes[bi].0 .0;
            let by = *boxes[bi].0 .1;

            set.insert(by);
            *map.entry(by).or_insert(0) += 1;
            bi += 1;
        }

        if let Some(&by) = set.range(cy..).next() {
            let count = map.get_mut(&by).unwrap();
            *count -= 1;
            if *count == 0 {
                map.remove(&by);
                set.remove(&by);
            }
        } else {
            println!("{}", NO);
            return;
        }
    }

    println!("{}", YES);
}
