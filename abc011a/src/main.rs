use proconio::input;
fn main() {
    input! {
        m: u32,
        n: u32,
        nn: u32,
    }
    let mut ans: u32 = nn;
    let mut u: u32 = nn;
    for _i in 0..nn {
        let mut cnt: u32 = 0;
        loop {
            if u < m {
                break;
            } else {
                u -= m;
                ans += n;
                cnt += 1;
            }
        }
        u += cnt * n;
        cnt = 0;
        if u < m {
            break;
        }
    }
    println!("{}", ans);
}
