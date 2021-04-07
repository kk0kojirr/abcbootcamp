use proconio::input;
fn main() {
    input! {
        mut a: u32,
        mut b: u32,
        k: u32,
    }
    for i in 0..k {
        if i % 2 == 0 {
            if a % 2 == 1 {
                a -= 1;
            } else {
            }
            a /= 2;
            b += a;
        } else {
            if b % 2 == 1 {
                b -= 1;
            } else {
            }
            b /= 2;
            a += b;
        }
    }
    println!("{} {}", a, b);
}
