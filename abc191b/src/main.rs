use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u32,
        a: [u32; n],
    }

    for v in a {
        if v != x {
            print!("{} ", v);
        }
    }
    println!("");
}
