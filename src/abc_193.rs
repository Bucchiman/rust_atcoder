/*
 * FileName:     abc_193
 * CreatedDate:  2022-07-03 21:39:23 +0900
 * LastModified: 2022-07-03 21:43:41 +0900
 */

use proconio::input;


pub fn abc_193_a(){
    input!{
        a:f32,
        b:f32,
    }
    println!("{}", (a-b)/a*100.0);
}
