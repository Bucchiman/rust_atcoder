use proconio::input;

fn main() {
    input!{
        n: i32,
        xy: [[f64; 2]; n],
    }
    let mut ans: f64 = 0.0;
    let mut i: usize;
    let mut j: usize;
    for i in 0..n{
        for j in i..n{
            let hoge = (xy[i as usize][0] - xy[j as usize][0])*(xy[i as usize][0] - xy[j as usize][0]) + (xy[i as usize][1] - xy[j as usize][1])*(xy[i as usize][1] - xy[j as usize][1]);
            if ans < hoge{
                ans = hoge;
            }
        }
    }
    println!("{}", ans.sqrt());
}
