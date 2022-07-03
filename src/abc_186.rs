/*
 * FileName:     abc_186
 * CreatedDate:  2022-06-21 21:44:37 +0900
 * LastModified: 2022-06-21 21:45:49 +0900
 */

use proconio::input;


pub fn abc_186_a(){
    input!{
        n: usize,
        w: usize,
    }
    println!("{}", n/w);
}
