/*
 * FileName:     abc_154
 * CreatedDate:  2022-07-04 08:21:23 +0900
 * LastModified: 2022-07-04 08:28:09 +0900
 */

use proconio::input;


pub fn abc_154_a(){
    input!{
        s: String,
        t: String,
        a: usize,
        b: usize,
        u: String,
    }
    if u == s{
        println!("{} {}", a-1, b);
    }
    else{
        println!("{} {}", a, b-1);
    }
}
