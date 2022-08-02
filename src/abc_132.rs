/*
 * FileName:     abc_132
 * CreatedDate:  2022-08-02 21:41:53 +0900
 * LastModified: 2022-08-02 21:48:51 +0900
 */

use proconio::input;


pub fn abc_132_b() {
    input!{
        n:i32,
        p:[i32; n],
    }
    let mut ans: i32 = 0;
    for i in 1..n-1{
        if p[(i-1) as usize] < p[i as usize] && p[i as usize] < p[(i+1) as usize]{
            ans += 1;
        }
        else if p[(i-1) as usize] > p[i as usize] && p[i as usize] > p[(i+1) as usize]{
            ans += 1;
        }
    }
    println!("{}", ans);
}
