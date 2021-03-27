use proconio::input;

fn main() {
    input! {
        n: u32,
        s: u32,
        d: u32,
        xy: [(u32, u32); n],
    }

    let mut res: u32 = 0;
    for (x, y) in xy {
        if x >= s || y <= d  {
            res = 0;
        } else {
            res = 1;
            break;
        }
    }

    if res == 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
