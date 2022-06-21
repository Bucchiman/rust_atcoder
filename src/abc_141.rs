/*
 * FileName:     abc_141
 * CreatedDate:  2022-06-18 20:36:47 +0900
 * LastModified: 2022-06-20 18:42:47 +0900
 */

use proconio::input;


pub fn abc_141_a(){
    input!{
        s: String,
    }
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b'S'{
            println!("Cloudy");
        }
        else if item == b'C'{
            println!("Rainy");
        }
        else{
            println!("Sunny");
        }
        break
    }
}

pub fn abc_141_b(){
    input!{
        s: String,
    }
    for (i, s_) in s.chars().enumerate(){
        if i % 2 == 0 && (s_ == 'R' || s_ == 'U' || s_ == 'D'){

        }
        else if i % 2 == 1 && (s_ == 'L' || s_ == 'U' || s_ == 'D'){

        }
        else{
            println!("No");
            return
        }
    }
    println!("Yes");
}
