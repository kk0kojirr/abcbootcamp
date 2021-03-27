use proconio::input;

fn main() {
    input! {
        x: [i32; 5],
    }
    for (i, v) in x.iter().enumerate() {
        if v == &0i32 {
            println!("{}", i+1);
            break;
        }
    }
}
