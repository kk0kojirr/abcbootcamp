fn main() {
    proconio::input! {
        s: String,
    }
    let mut dp = [0; 5];
    dp[0] = 24;
    let d = [1, 1, 2, 6, 24];

    for c in s.chars().filter(|c| *c != 'x') {
        let s = (c == 'o') as usize;
        //
        println!("##### c == 'o' is {} {}", c, s);

        let mut next = [0; 5];
        for (i, dp) in dp.iter().enumerate() {
            for j in s..=(4 - i) {
                next[i + j] += *dp / d[j];
                println!("dp{}  d[j]{} = {}", *dp, d[j], *dp / d[j]);
                println!("i{}  j{}  d[j]:{} dp {:?} next {:?}",i,j, d[j],dp, next)
            }
        }
        dp = next;
    }
    println!("{}", dp[4]);
}