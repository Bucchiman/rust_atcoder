/*
 * FileName:     abc_254
 * CreatedDate:  2022-06-14 22:58:22 +0900
 * LastModified: 2022-06-14 23:07:57 +0900
 */

use proconio::input;


pub fn abc_254_a() {
    input!{
        n: usize,
    }
    let stri: &str = &n.to_string();
    println!("{}", &stri[1..3]);
}
