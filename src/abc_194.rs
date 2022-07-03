/*
 * FileName:     abc_194
 * CreatedDate:  2022-07-03 21:01:37 +0900
 * LastModified: 2022-07-03 21:12:33 +0900
 */

use proconio::input;


pub fn abc_194_a() {
    input!{
        a:usize,
        b:usize,
    }
    if (a+b>=15 && b>=8){
        println!("1");
    }
    else if (a+b>=10 && b>=3){
        println!("2");
    }
    else if (a+b>=3){
        println!("3");
    }
    else{
        println!("4");
    }

}
