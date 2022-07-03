/*
 * FileName:     abc_160
 * CreatedDate:  2022-06-21 20:31:38 +0900
 * LastModified: 2022-06-21 20:54:58 +0900
 */

use proconio::input;


pub fn abc_160_a(){
    input!{
        x: usize,
    }
    println!("{}", (x/500)*1000+(x%500)/5*5);
}
