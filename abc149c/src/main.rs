use proconio::input;
fn is_prime(n: usize) -> bool {
    let mut i = 2;
    while i*i <= n {
        if n % i == 0 {
            return false;
        }
        i+=1;
    }
    return true;
}
fn main() {
    input! {
        mut x: usize,
    }
    while !is_prime(x) {
        x+=1;
    }
    println!("{}", x);
}
