use proconio::input;
fn main() {
    input!{
        n: usize,
        mut v: [u32; n],
    }
    v.sort();
    println!("{:?}", v);
}
