use proconio::input;
fn main() {
    input! {
        n: usize,
        mut st:[(String, u32);n],
    }
    st.sort_by(|a, b| a.1.cmp(&b.1));
    st.reverse();
    println!("{}", st[1].0);
}
