/*
 * FileName:     abc_223
 * CreatedDate:  2022-07-26 23:04:41 +0900
 * LastModified: 2022-07-26 23:50:07 +0900
 */

use proconio::input;


pub fn abc_223_c() {
    input!{
        n: i32,
        ab: [[f64; 2]; n],
    }
    let mut total_sec: f64 = 0.0;
    for _ab in &ab{
        total_sec += _ab[0] / _ab[1];
    }
    let mut i:usize = 0;
    let mut current_sec: f64 = 0.0;
    let mut ans: f64 = 0.0;
    while(current_sec<total_sec/2.0){
        if current_sec + ab[i][0] / ab[i][1] >= total_sec / 2.0{
            ans += (total_sec/2.0-current_sec)*ab[i][1];
            break;
        }
        else{
            current_sec += ab[i][0] / ab[i][1];
            ans += (ab[i][0] as f64);
            i += 1;
        }
    }
    println!("{}", ans);
}
