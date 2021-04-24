use proconio::input;
fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
    }
    println!("{}", if a*a + b*b < c*c {"Yes"} else {"No"});
}
