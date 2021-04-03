use proconio::input;
fn main() {
    input! {
        q: u32,
    }
    println!("{}", if q == 1 {"ABC"} else if q == 2 {"chokudai"} else {""});
}
