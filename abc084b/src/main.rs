use proconio::input;
fn main() {
    input! {
        a: usize,
        b: usize,
        s: String,
    }
    let cc = s.chars().collect::<Vec<char>>();
    let mut ans = true;
    for i in 0..(a+b+1) {
        if i < a {
            if cc[i] as u8 >= b'0' && cc[i] as u8 <= b'9' {
                continue;
            } else {
                ans = false;
                break;
            }
        }
        if i == a {
            if cc[i] == '-' {
                continue;
            } else {
                ans = false;
                break;
            }
        }
        if i > a {
            if cc[i] as u8 >= b'0' && cc[i] as u8 <= b'9' {
                continue;
            } else {
                ans = false;
                break;
            }
        }
    }
    println!("{}", if ans {"Yes"} else {"No"});
}
