use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    }
    let mut ans = 0;
    for x in 0..=9999 {
        let v = [x%10, x/10%10, x/100%10, x/1000];
        let mut flag = true;
        for (i, &c) in s.iter().enumerate() {
            match c {
                'x' => {flag &= !v.contains(&i);},
                'o' => {flag &= v.contains(&i);},
                _ => {},
            }
        }
        if flag {ans+=1;}
    }
    println!("{}", ans);
}