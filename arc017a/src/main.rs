use proconio::input;
fn main() {
    input! {
        n: u32,
    }
    println!("{}", if is_prime(n) {"YES"} else {"NO"});
}

fn is_prime(n: u32) -> bool {
    if n % 2 == 0 {
        return false;
    }

    let mut i = 2;
    while i*i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}