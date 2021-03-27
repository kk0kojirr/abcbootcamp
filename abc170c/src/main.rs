use proconio::input;

fn main() {
    input! {
        x: u32,
        n: usize,
        p: [u32; n],
    }

    let mut min: u32 = 1000;
    let mut ans: u32 = 1000;
    for v in (0..=105).rev() {
        if p.iter().any(|&k| k == v) {
            continue;
        }

        if min >= ((x as i32).abs() - (v as i32)).abs() as u32 {
            ans = v;
            min = ((x as i32).abs() - (v as i32)).abs() as u32;
        }

    }

    println!("{}", ans);
}
