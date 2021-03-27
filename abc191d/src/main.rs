use proconio::input;

fn main() {
    input! {
        x:f64,
        y:f64,
        r:f64,
    }
    let Ri64 = r as i64;
    let mut Xi = (x * 10000f64).round() as i64 % 10000;
    let mut Yi = (y * 10000f64).round() as i64 % 10000;
    let mut Ri = (r * 10000f64).round() as i64;
    let Ri2 = Ri * Ri;
    let mut ans = 0i64;

    for x_ in (-Ri64 - 3)..=(Ri64 + 3) {
        let x = x_ * 10000;
        let mut yy = Ri2 - (x - Xi) * (x - Xi);
        if yy < 0 {
            continue;
        }
        if yy == 0 {
            if Yi == 0 {
                ans += 1;
            }
        } else if yy > 0 {
            let mut y = (yy as f64).sqrt() as i64;
            if y * y > yy {
                y -= 1;
            }
            let mut y1 = (Yi + y) / 10000;
            let mut y0 = (Yi - y) / 10000;
            if y1 * 10000 > Yi + y {
                y1 -= 1;
            }
            if y0 * 10000 >= Yi - y {
                y0 -= 1;
            }
            ans += y1 - y0;
        }
    }
    println!("{}", ans);
}
