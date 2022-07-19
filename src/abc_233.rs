/*
 * FileName:     abc_233
 * CreatedDate:  2022-07-19 23:51:57 +0900
 * LastModified: 2022-07-20 00:01:29 +0900
 */

use proconio::input;


pub fn abc_233_b() {
    input!{
        l: usize,
        r: usize,
        s: String,
    }
    println!("{}{}{}", &s[0..(l-1)], &s[(l-1)..r].chars().rev().collect::<String>(), &s[r..s.len()]);
}
