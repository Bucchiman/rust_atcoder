/*
 * FileName:     abc_221_a
 * CreatedDate:  2022-06-10 22:10:39 +0900
 * LastModified: 2022-06-10 22:18:01 +0900
 */

use proconio::input;


pub fn abc_221_a() {
    input! {
        a: u32,
        b: u32,
    }
    println!("{}", 32_i32.pow(a-b))
}
