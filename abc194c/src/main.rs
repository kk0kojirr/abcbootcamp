use proconio::input;
fn main() {
    input! {
        n:usize,
        a:[i64; n],
    }
    let mut ans = 0;
    let max_a: i64 = 200; // -200 =< k =< 200
    let mut d = [0; 401];
    for i in 0..n {
        for aj in -1 * max_a..=max_a { // -200 to 200
            let c = d[(max_a + aj) as usize];
            let x = a[i] - aj;
            ans += x * x * c;
        }
        d[(max_a + a[i]) as usize] += 1; 
    }
    println!("{}", ans);
}
