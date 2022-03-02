use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! { N: usize }

    // let mut names = vec![vec!["".to_string(); 2]; N];
    let mut ss = vec!["".to_string(); N];
    let mut ts = vec!["".to_string(); N];
    let mut ans = "No";
    for i in 0..N {
        input! { S: String, T:String }
        ss[i] = S;
        ts[i] = T;
        for j in 0..i {
            if ss[j] == ss[i] && ts[j] == ts[i] {
                ans = "Yes";
                break;
            }
        }
    }
    println!("{}", ans);
}
