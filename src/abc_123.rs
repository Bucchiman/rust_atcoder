/*
 * FileName:     abc_123
 * CreatedDate:  2022-07-07 08:04:52 +0900
 * LastModified: 2022-07-07 22:45:34 +0900
 */

use proconio::input;


pub fn abc_123_b(){
    input!{
        a_e: [i32; 5],
    }
    let mut tmp_box: [i32; 2] = [5, 10];
    for (i, ae) in a_e.iter().enumerate(){
        if ae % 10 != 0 && ae % 10 < tmp_box[1]{
            tmp_box[0] = i as i32;
            tmp_box[1] = ae % 10;
        }
    }
    let mut ans: i32 = 0;
    for (i, ae) in a_e.iter().enumerate(){
        if (i as i32) == tmp_box[0]{
            ans += ae;
        }
        else{
            if ae % 10 == 0 {
                ans += ae;
            }
            else{
                ans = ans + (ae - ae % 10) + 10;
            }
        }
        println!("{}", ans);
    }
}
