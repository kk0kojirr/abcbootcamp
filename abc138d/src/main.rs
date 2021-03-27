use proconio::{input, fastout};
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        px: [(Usize1, usize); q]
    }

    // c[i]には接続されている頂点がpushされている
    // i番目の辺は頂点a, bを結ぶ
    let mut c = vec![vec![];n];
    for &(a, b) in &ab {
        c[a].push(b);
        c[b].push(a);
    }
    // Usize1しているので、入力値−1
    // 根のindexを0にしたい
    // println!("{:?}", ab);
    // cのvec中身をprint
    // println!("{:?}", c);

    // n個ある頂点のカウンター
    let mut counter = vec![0; n];
    // 頂点pを根にxを加算
    for &(root, x) in &px {
        counter[root] += x;
    }

    // dfs 深さ優先
    // 頂点番号indexでcounter, visitedの属性を持つ
    let mut visited = vec![false; n];
    let mut stack = vec![];

    // 0, 0で頂点1
    stack.push((0, 0));
    while let Some((node, value)) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        counter[node] += value;
        // 繋がっている頂点をpush
        for &child in &c[node] {
            stack.push((child, counter[node]));
        }
    }

    // ans
    for i in 0..n-1 {
        print!("{} ", counter[i]);
    }
    println!("{}", counter[n-1]);
}
