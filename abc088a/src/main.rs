use proconio::input;
fn main() {
    input! {
        n: u32,
        a: u32,
    }
    let need = n % 500;
    println!("{}", if need <= a {"Yes"} else {"No"});
}
