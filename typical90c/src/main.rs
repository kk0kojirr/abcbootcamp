use proconio::input;
use std::collections::VecDeque;
static INF: u32 = std::u32::MAX;
fn main() {
    input! {
        n: usize,
        ab: [(u32, u32); n-1],
    }
    let mut g = Vec::<Vec::<u32>>::new();
    for _ in 0..=n {
        g.push(Vec::<u32>::new());
    }
    // create graph data
    for abb in &ab {
        g[(abb.0) as usize].push(abb.1);
        g[(abb.1) as usize].push(abb.0);
    }
    // println!("{:?}", g);

    let mut dist = Vec::<u32>::new();
    for _ in 0..=n {
        dist.push(INF);
    }
    // distance from 1
    getdist(1,&mut dist, &g, n);
    let mut maxi = 0;
    let mut p = 0;
    for i in 0..=n {
        if p < dist[i] {
            p = dist[i];
            maxi = i;
        }
    }
    // println!("max:{} dist:{:?}", maxi, dist);

    // distance from maxi
    getdist(maxi as u32,&mut dist, &g, n);
    dist.sort();
    // println!("{:?}", dist);
    println!("{}", dist[n]+1);
}

fn getdist(s: u32, dist: &mut Vec<u32>, g: &Vec<Vec<u32>>, n: usize) {
    for i in 0..=n {
        if i == 0 {
            dist[i] = 0;
        } else {
            dist[i] = INF;
        }
    }
    let mut q = VecDeque::<u32>::new();
    q.push_back(s);
    dist[s as usize] = 0;

    // println!("{:?}", g);
    while !q.is_empty() {
        if let Some(&pos) = q.front() {
            q.pop_front();
            for to in &g[pos as usize] {
                if dist[(*to) as usize] == INF {
                    dist[(*to) as usize] = dist[pos as usize] + 1;
                    q.push_back(*to);
                }
            }
        }
    }
    // println!("{:?}", dist);
}