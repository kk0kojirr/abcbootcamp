use proconio::input;
use itertools::Itertools;
fn main() {
    input! {
        n: i64,
        mut xy: [(i64, i64); n],
    }
    let mut maxx = std::i64::MIN;
    let mut minx = std::i64::MAX;
    let mut maxy = std::i64::MIN;
    let mut miny = std::i64::MAX;
    let mut secmaxx = std::i64::MIN;
    let mut secminx = std::i64::MAX;
    let mut secmaxy = std::i64::MIN;
    let mut secminy = std::i64::MAX;

    for vv in &xy {
        minx = std::cmp::min(minx, vv.0);
        miny = std::cmp::min(miny, vv.1);
        maxx = std::cmp::max(maxx, vv.0);
        maxy = std::cmp::max(maxy, vv.1);
    }

    for vv in &xy {
        if vv.0 > minx {
            secminx = std::cmp::min(secminx, vv.0);
        }
        if vv.1 > miny {
            secminy = std::cmp::min(secminy, vv.1);
        }
        if vv.0 < maxx {
            secmaxx = std::cmp::max(secmaxx, vv.0);
        }
        if vv.1 < maxy {
            secmaxy = std::cmp::max(secmaxy, vv.1);
        }
    }

    let mut chxy:Vec<(i64, i64)> = Vec::<(i64, i64)>::new();
    for vv in &xy {
        if maxx == vv.0 || secmaxx == vv.0 || maxy == vv.1 || secmaxy == vv.1 ||
           minx == vv.0 || secminx == vv.0 || miny == vv.1 || secminy == vv.1 {
            chxy.push((vv.0, vv.1));
        }
    }

    let mut lon:Vec<i64> = Vec::<i64>::new();
    for xy in chxy.iter().combinations(2) {
        lon.push(std::cmp::max((xy[0].0 - xy[1].0).abs(), (xy[0].1 - xy[1].1).abs()));
    }
    lon.sort();
    lon.reverse();

    println!("{}", lon[1]);
}
