/*
 * FileName:     abc_187
 * CreatedDate:  2022-06-21 21:38:49 +0900
 * LastModified: 2022-06-21 21:43:03 +0900
 */

use proconio::input;


pub fn abc_187_a() {
    input!{
        a: usize,
        b: usize,
    }
    if a/100+a%100/10+a%100%10 > b/100+b%100/10+b%100%10{
        println!("{}", a/100+a%100/10+a%100%10);
    }
    else{
        println!("{}", b/100+b%100/10+b%100%10);
    }
}
