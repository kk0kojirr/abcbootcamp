use proconio::input;
fn main() {
    input! {
        n: u32,
    }
    if n % 2 == 1 {
        println!("");
        return;
    }

    let m = 2u64.pow(n);
    let mut ptn = vec![];
    for i in 0..m {
        let p = format!("{:0>nn$b}", i, nn=n as usize);
        if check(&p) {
            ptn.push(p);
        }
    }
    ptn.reverse();
    for p in ptn {
        println!("{}", p.replace("1", "(").replace("0", ")"));
    }
}

fn check(p:&String) -> bool {
    let mut dep = 0;
    for c in p.chars() {
        if c == '1' { dep += 1; }
        if c == '0' { dep -= 1; }
        if dep < 0 {return false;}
    }
    if dep == 0 { return true; }
    return false;
}