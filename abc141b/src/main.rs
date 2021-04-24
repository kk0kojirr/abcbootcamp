use proconio::input;
fn main() {
    input! {
        s: String,
    }
    let mut ok = true;
    for (i, c) in s.chars().enumerate() {
        if (i+1) % 2 == 0 {
            match c {
                'L' => ok = true,
                'U' => ok = true,
                'D' => ok = true,
                _ => ok = false,
            }
            if !ok {break;}
        } else {
            match c {
                'R' => ok = true,
                'U' => ok = true,
                'D' => ok = true,
                _ => ok = false,
            }
            if !ok {break;}
        }
    }
    println!("{}", if ok {"Yes"} else {"No"});
}
