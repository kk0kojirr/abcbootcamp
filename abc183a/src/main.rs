fn main() {
    proconio::input! {
        x: i32
    }
    let mut relu: i32 = 0;
    if x >= 0 {
        relu = x;
    } else {
        relu = 0;
    }
    println!("{}", relu);
}
