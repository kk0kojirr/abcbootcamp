use proconio::input;
fn main() {
    input! {
        s: String,
    }
    let mut ok: u8 = 0;
    for c in s.chars() {
        if c == 'C' {
            ok = 1;
        }

        if c == 'F' && ok == 1 {
            ok = 2;
        }
    }
    println!("{}", if ok == 2 {"Yes"} else {"No"});
}
