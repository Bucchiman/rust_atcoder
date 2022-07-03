/*
 * FileName:     abc_183
 * CreatedDate:  2022-07-03 21:46:11 +0900
 * LastModified: 2022-07-03 21:53:09 +0900
 */

use proconio::input;



pub fn abc_183_a(){
    input!{
        s_x: f32,
        s_y: f32,
        g_x: f32,
        g_y: f32,
    }
    // y = (g_y+s_y)/(g_x-s_x)(x-g_x)+g_y
    println!("{}", -g_y*(g_x-s_x)/(g_y+s_y)+g_x);
}
