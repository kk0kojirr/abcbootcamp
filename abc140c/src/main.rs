use proconio::input;
fn main() {
    input! {
        n: usize,
        b: [usize; n-1],
    }
    let mut a = vec![1000000; n];
    for i in 0..2 {
        for (a, b) in a[i..].iter_mut().zip(b.iter()) {
            println!("{} {} {}", a, b, a[i..]);
            *a = (*a).min(*b);
        }
    }
    println!("{}", a.iter().sum::<usize>());
}
