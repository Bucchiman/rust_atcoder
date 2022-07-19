/*
 * FileName:     abc_243
 * CreatedDate:  2022-07-19 23:12:36 +0900
 * LastModified: 2022-07-19 23:27:34 +0900
 */

use proconio::input;


pub fn abc_243_b() {
    input!{
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    let mut ans: [i32; 2] = [0, 0];
    for (i, _a) in a.iter().enumerate(){
        if (_a == &b[i]){
            ans[0] += 1;
        }
        else if (_a != &b[i] && b.iter().any(|e| e == _a)){
            ans[1] += 1;
        }
    }
    println!("{}\n{}", ans[0], ans[1]);
}
