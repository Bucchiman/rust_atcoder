/*
 * FileName:     abc_237
 * CreatedDate:  2022-08-04 23:07:39 +0900
 * LastModified: 2022-08-04 23:35:25 +0900
 */

use proconio::input;


pub fn abc_237_c() {
    input!{
        mut s:String,
    }
    let mut i:i32 = 0;
    let s_len:i32 = s.len().try_into().unwrap();
    let vec: Vec<String> = s.iter().map(|&_s| _s.to_string()).collect();
    /*(while(i <= s_len){
        s = "a".to_owned() + &s;
        for j in 0..(s_len+i)/2{

        }
        i += 1;
    }*/
}
