use proconio::input;

fn main() {
    input! {
        a: u32,
        mut p: u32,
    }

    p += a * 3;

    if p % 2 == 1 {
        p -= 1;
    }

    println!("{}", p / 2);
}
