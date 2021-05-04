use proconio::input;

fn main() {
    input! {
        nko: usize,
        lcm: i64,
        kchoice: u32,
        a: [i64;nko],
    }
    let mut left:i64 = -1;
    let mut right:i64 = lcm+1;
    while right-left>1 {
        let mid = left+(right-left) /2;
        if !solve(mid, &a, nko, lcm, kchoice) {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{:?}", left);
}

fn solve(m:i64, a: &Vec<i64>, n:usize, lcm:i64, kchoice:u32) -> bool {
    let mut cuts = 0;
    let mut prev = 0;
    for i in 0..n {
        // mcm以上にできる？残りもmcm以上？
        if a[i] - prev >= m && lcm -a[i] >= m {
            cuts += 1;
            prev = a[i];
        }
    }
    if cuts >= kchoice {
        return true;
    } else {
        return false;
    }
}