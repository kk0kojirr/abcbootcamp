use proconio::input;
fn main() {
    input! {
        s: String,
    }
    let mut ans = 999;
    for ss in s.chars().collect::<Vec<char>>().windows(3) {
        let num = (ss[0] as i32 - 48) * 100 +
                  (ss[1] as i32 - 48) * 10 +
                  (ss[2] as i32 - 48);
        ans = std::cmp::min(ans, (753-num).abs());
    }
    println!("{}", ans);
}
