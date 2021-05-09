use proconio::input;
fn main() {
    input! {
        s: String,
    }
    let mut alp = vec![0; 26];
    for c in s.chars() {
        alp[c as usize - b'a' as usize] += 1;
    }
    println!("{}", if alp.into_iter().any(|x| x>1) {"no"} else {"yes"});
}
