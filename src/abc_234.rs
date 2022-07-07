/*
 * FileName:     abc_234
 * CreatedDate:  2022-07-07 22:58:34 +0900
 * LastModified: 2022-07-07 23:23:54 +0900
 */

use proconio::input;


pub fn abc_234_b(){
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
