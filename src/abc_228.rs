/*
 * FileName:     abc_228
 * CreatedDate:  2022-06-21 16:22:28 +0900
 * LastModified: 2022-06-21 16:29:39 +0900
 */

use proconio:: input;

pub fn abc_228_a(){
    input!{
        s: mut i32,
        t: mut i32,
        x: mut i32
    }
    if s > t{
        t += 24;
    }
    for i in s..t{
        println!("{}", i);
    }
}
