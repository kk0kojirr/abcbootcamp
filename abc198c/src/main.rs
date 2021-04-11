use proconio::input;
fn main() {
    input! {
        r: i128,
        x: i128,
        y: i128,
    }
    let ju = (0-x).pow(2) + (0-y).pow(2);

    if r.pow(2) > ju {
        println!("{}", 2);
        return;
    }

    let mut ans = 0;
    while ju > (r * ans).pow(2) {
        ans += 1;
    }
    println!("{}", ans);
}
