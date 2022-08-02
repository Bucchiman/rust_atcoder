/*
 * FileName:     abc_222
 * CreatedDate:  2022-07-30 10:00:58 +0900
 * LastModified: 2022-07-30 10:26:17 +0900
 */

use proconio::input;


pub fn abc_222_c() {
    input!{
        n: i32,
        m: i32,
        a: [String; 2*n],
    }

    for i in 0..2*n{
        println!("{}", a[i as usize]);
    }
}
