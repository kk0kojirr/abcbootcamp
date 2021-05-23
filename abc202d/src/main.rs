use proconio::input;
fn main() {
    input! {
        mut a: i64,
        mut b: i64,
        mut k: i64,
    }
    let mut c = vec![vec![0; 61]; 61];
    c[0][0] = 1 as i64;
    for i in 0..60 {
        for j in 0..i+1 {
            c[i+1][j] += c[i][j];
            c[i+1][j+1] += c[i][j];
        }
    }

    let mut ans = String::new();
    while a+b > 0 {
        let mut x: i64 = 0;
        if a > 0 {
            x = c[(a+b-1) as usize][(a-1) as usize];
        }
        if k <= x {
            ans = ans + &'a'.to_string();
            a -= 1;
        } else {
            ans = ans + &'b'.to_string();
            b -= 1;
            k -= x;
        }
    }
    println!("{}", ans);
}
