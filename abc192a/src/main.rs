use proconio::input;

fn main() {
    input! {
        x: u32,
    };

    let mut i: u32 = x;
    let mut ans: u32 = 0;

    loop {
        i += 1;
        ans += 1;
        if i % 100 == 0 {
            break;
        }
    }

    println!("{}", ans);
}
