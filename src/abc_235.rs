/*
 * FileName:     abc_235
 * CreatedDate:  2022-07-17 14:20:48 +0900
 * LastModified: 2022-07-17 14:32:43 +0900
 */

use proconio::input;


pub fn abc_235_b() {
    input!{
        n: usize,
        h: [i32; n],
    }
    let mut pre_h: i32 = 0;
    for (i, _h) in h.iter().enumerate(){
        if (i==0){
            pre_h = *_h;
            continue;
        }
        if (pre_h < *_h){
            pre_h = *_h;
        }
        else{
            break;
        }
    }
    println!("{}", pre_h);
}
