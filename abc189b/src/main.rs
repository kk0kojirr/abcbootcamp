use proconio::input;
fn main() {
    input! {
        n: usize,
        x: u32,
        vp: [(u32, u32); n],
    }
    let mut ans: i32 = 0;
    let mut cur: u32 = 0;
    for (vol, alp) in vp {
        ans += 1;
        cur += vol * alp;
        if cur > x * 100 {
            break;
        }
    }
    if cur <= x * 100 {
        ans = -1;
    } 

    println!("{}", ans);
}
