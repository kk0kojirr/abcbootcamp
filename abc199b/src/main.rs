use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [u32; n],
        b: [u32; n],
    }
    let mut maxa = 0;
    let mut minb = 100100;
    for i in 0..n {
        maxa = std::cmp::max(maxa, a[i]);
        minb = std::cmp::min(minb, b[i]);
    }
    if minb < maxa {
        println!("{}", 0);
    } else {
        println!("{}",minb-maxa+1);
    }
}
