use proconio::input;
fn main() {
    input! {
        k: u32,
        n: usize,
        mut a: [u32; n]
    }
    let mut kukan = vec![0; n];
    let mut i = 0;
    for p in a.windows(2) {
        kukan[i] = p[1] - p[0];
        i += 1;
    }
    kukan[n-1] = k-a[n-1] + a[0];

    let mut ans = 0;
    let mut max: u32 = 0;
    for ku in kukan {
        ans += ku;
        max = std::cmp::max(max, ku);
    }
    println!("{}", ans - max);
}
