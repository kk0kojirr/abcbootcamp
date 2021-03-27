use proconio::input;

fn main() {
    input! {
        mut a: u32,
        mut b: u32,
        c: u32,
    }

    if c == 0 {
        // Takahasi
        if a > b {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    } else {
        // Aoki
        if b > a {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    }
}
