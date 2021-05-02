use proconio::input;
fn main() {
    input! {
        s: String,
    }
    let mut ans = 0;
    let mut next = 0;

    for i in 0..12-3 {
        if (next == 0 || next+1 == i) &&
           s.chars().nth(i).unwrap()   == 'Z' &&
           s.chars().nth(i+1).unwrap() == 'O' &&
           s.chars().nth(i+2).unwrap() == 'N' &&
           s.chars().nth(i+3).unwrap() == 'e' {
            ans += 1;
            next = i+3;
        } else {
            next = 0;
        }
    }
    println!("{}", ans);
}

// ZONeZOdeZONe