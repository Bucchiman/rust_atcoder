/*
 * FileName:     abc_138
 * CreatedDate:  2022-07-04 08:05:31 +0900
 * LastModified: 2022-07-04 08:18:04 +0900
 */

use proconio::input;


pub fn abc_138_a(){
    input!{
        n: usize,
        mut a: [f32; n],
    }
    let mut ans = 0.0;
    for _a in a{
        ans = ans + 1.0/_a;
    }
    println!("{}", 1.0/ans);
}
