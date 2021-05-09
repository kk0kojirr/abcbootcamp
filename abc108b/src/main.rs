use proconio::input;
fn main() {
    input! {x1: i32, y1: i32, x2: i32, y2: i32, }
    let dx = x2 -x1;
    let dy = y2 -y1;
    println!("{} {} {} {}", x2-dy, y2+dx, x1-dy, y1+dx);
}
