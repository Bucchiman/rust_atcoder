/*
 * FileName:     abc_246
 * CreatedDate:  2022-07-29 21:59:19 +0900
 * LastModified: 2022-07-29 22:09:41 +0900
 */

use proconio::input;


pub fn abc_246_b() {
    input!{
        a: f64,
        b: f64,
    }
    println!("{} {}", a/(a*a+b*b).sqrt(), b/(a*a+b*b).sqrt())
}
