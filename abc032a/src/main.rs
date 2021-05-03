use proconio::input;
fn main() {
    input! {
        a: u32,
        b: u32,
        n: u32,
    }
    for i in n.. {
        if i%a == 0 && i%b == 0 {
            println!("{}", i);
            return;
        }
    }
}
