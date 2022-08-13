/*
 * FileName:     abc_229
 * CreatedDate:  2022-08-05 22:54:26 +0900
 * LastModified: 2022-08-05 23:54:10 +0900
 */

use proconio::input;


pub fn abc_229_b() {
    input!{
        a: String,
        b: String,
    }
    let a_invert: String = a.chars().rev().collect();
    let b_invert: String = b.chars().rev().collect();
    for i in 0..min(a_invert.len(), b_invert.len()){
        println!("{}, {}", a_invert[i], b_invert[i]);
    }
}
