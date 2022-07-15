/*
 * FileName:     abc_240
 * CreatedDate:  2022-07-15 08:21:34 +0900
 * LastModified: 2022-07-15 08:26:52 +0900
 */

use proconio::input;


pub fn abc_240_a() {
    input!{
        a: i32,
        b: i32,
    }
    if ((-1 <= (a-b) && (a-b) <= 1) || (a == 1 && b == 10) || (b == 1 && a == 10)){
        println!("Yes");
    }
    else{
        println!("No");
    }
}
