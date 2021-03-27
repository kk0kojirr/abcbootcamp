fn main() {
    proconio::input! {
        a: u32,
        s: String,
    }
    println!("{}",
    if a >= 3200 { s } else { String::from("red") });
}
