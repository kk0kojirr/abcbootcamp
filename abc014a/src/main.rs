use proconio::input;
fn main() {
    input! {
        mut a: u32,
        mut b: u32,
        mut c: u32,
    }
    let mut ans: i64 = 0;
    loop {
        if a % 2 == 1 || b % 2  == 1 || c % 2 == 1 {
            break;
        }
        if a == b && b == c {
            ans = -1;
            break;
        }

        let tmpa = a / 2;
        let tmpb = b / 2;
        let tmpc = c / 2;
        a = tmpb + tmpc;
        b = tmpa + tmpc;
        c = tmpa + tmpb;

        ans += 1;
    }
    println!("{}", ans);
}
