use proconio::input;
fn main() {
    input! {
        n: usize,
        t: [i32; n],
        m: usize,
        px: [(usize, i32); m],
    }

    let mut sum: i32 = 0;
    for time in &t {
        sum += time;
    }

    for (pidx, newtime) in px {
        let oldtime = t[pidx-1];
        let jit = oldtime - newtime;
        println!("{}", sum - jit);
    }
}
