use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut res : u32 = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            if n == i * j {
                res = 1;

            }
        }
    }

    if res == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
