use proconio::input;

fn main() {
    input! {
        x: u32,
        y: u32,
    }

    let mut ans: bool = false;

    for i in 0..=x {
        let turu = i;
        let kame = x - i;

        if turu * 2 + kame * 4 == y {
            ans = true;
            break;
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
