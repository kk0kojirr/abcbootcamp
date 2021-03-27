fn main() {
    proconio::input! {
        n: u32,
        mut a: [u32; n],
    }
    a.sort();
    let mut alice: u32 = 0;
    let mut bob: u32 = 0;

    for (i, v) in a.iter().rev().enumerate() {
        if i % 2 == 0 {
            alice += v;
        } else {
            bob += v;
        }
    }
    println!("{}", alice - bob);
}
