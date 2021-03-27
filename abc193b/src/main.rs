use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        shops: [(u64, u64, u64); n],
    }

    let mut ans: i64 = -1;

    for shop in shops {
        let a = shop.0;
        let p = shop.1;
        let x = shop.2;

        let buys: u64 = ((a * 10) + 5) / 10;
        
        if x as i64 - buys as i64 > 0 {
            if ans != -1 {
                ans = cmp::min(ans, p as i64);
            } else {
                ans = p as i64;
            }
        } else {
            continue;
        }
    }
    println!("{}", ans);
}
