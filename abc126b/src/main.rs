use proconio::input;
use proconio::marker::Chars;
fn main() {
    input!(s: Chars);

    let (h, l): (u8, u8) = (
        s[..=1].iter().collect::<String>().parse::<u8>().unwrap(),
        s[2..].iter().collect::<String>().parse::<u8>().unwrap()
    );
    let mut ans = "NA";
    if (h > 12 || h == 0) && (l > 12 || l == 0) {
        ans = "NA";
    } else if (h <= 12 && h != 0) && (l <= 12 && l != 0) {
        ans = "AMBIGUOUS";
    } else if (h <= 12 && h != 0) && (l > 12 || l == 0) {
        ans = "MMYY";
    } else if (h > 12 || h == 0) && (l <= 12 && l != 0) {
        ans = "YYMM";
    }

    println!("{}", ans);
}
