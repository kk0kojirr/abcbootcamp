use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        ccc: Chars,
        ddd: Chars,
    }
    println!("{}", if ccc[2] == ddd[0] && ccc[1] == ddd[1] && ccc[0] == ddd[2] {"YES"} else {"NO"});
}
