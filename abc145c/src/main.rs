use itertools::Itertools;
use proconio::input;
 
fn main() {
    // 順列の数はN!で表現できる
    // println!("{}", (1..=4).product::<usize>());

    input! {
        n: usize,
        xy: [(f64, f64); n],
    }
    
    // 順列をpermutationsで生成してmap
    let ans = xy
        .iter()
        .permutations(n)
        .map(|e| {
            e.windows(2)
                .map(|e| ((e[0].0 - e[1].0).powf(2.0) + (e[0].1 - e[1].1).powf(2.0)).sqrt())
                .sum::<f64>()
        })
        .sum::<f64>()
        / (1..=n).product::<usize>() as f64;

    println!("{}", ans);
}