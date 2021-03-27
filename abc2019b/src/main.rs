use proconio::input;
fn main() {
    input! {
        n: u32,
    }

    let mut ok: bool = false;
    for v in (n - n / 10)..=n {
        if n == v * 108 / 100 {
            ok = true;
            println!("{}", v);
        }
    }
    if !ok {
        println!(":(");
    }
}
