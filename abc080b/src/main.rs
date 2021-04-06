use proconio::input;
fn main() {
    input! {
        n: String,
    }
    let mut ans: u32 = 0;
    let x: u32 = n.parse().unwrap();
    for num in n.chars() {
        ans += num as u32 - 48;
    }
    println!("{}", if x % ans == 0 {"Yes"} else {"No"});
}
