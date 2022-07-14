/*
 * FileName:     abc_241
 * CreatedDate:  2022-07-15 08:12:24 +0900
 * LastModified: 2022-07-15 08:18:30 +0900
 */

use proconio::input;


pub fn abc_241_a() {
    input!{
        a: [i32; 10],
    }
    let mut i: i32 = 0;
    let mut idx: i32 = 0;
    while (i < 3){
        idx = a[idx as usize];
        i += 1;
    }
    println!("{}", idx);
}
