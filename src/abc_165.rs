/*
 * FileName:     abc_165
 * CreatedDate:  2022-06-21 21:47:49 +0900
 * LastModified: 2022-06-21 21:59:31 +0900
 */

use proconio::input;


pub fn abc_165_a() {
    input!{
        k: usize,
        a: usize,
        b: usize,
    }
    if a/k*k >= a || a/k*k+k<=b{
        println!("OK");
    }
    else{
        println!("NG");
    }
}
