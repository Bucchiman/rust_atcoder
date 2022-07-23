/*
 * FileName:     abc_249
 * CreatedDate:  2022-07-22 08:34:23 +0900
 * LastModified: 2022-07-23 17:36:39 +0900
 */

use proconio::input;


pub fn abc_249_b() {
    input!{
        s: String,
    }
    let mut ans: [i32; 2] = [0, 0];
    let mut tmp = vec![];
    let mut flag:i32 = 0;
    for _s in s.chars(){
        if tmp.iter().any(|e| e==&(_s as u8)){
            flag = 1;
        }
        if 65 <= (_s as u8) && (_s as u8) <= 65+26{
            ans[0] = 1;
        }
        else{
            ans[1] = 1;
        }
        tmp.push(_s as u8);
    }
    if ans[0] == 1 && ans[1] == 1 && flag == 0{
        println!("Yes");
    }
    else{
        println!("No");
    }
}
