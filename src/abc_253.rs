/*
 * FileName:     abc_253
 * CreatedDate:  2022-07-23 17:39:55 +0900
 * LastModified: 2022-07-23 22:19:23 +0900
 */

use proconio::input;


pub fn is_o(_s: char) -> bool{
    match _s{
        'o' => true,
        '-' => false,
        _ => todo!(),
    }
}


pub fn abc_253_b() {
    input!{
        h: i32,
        w: i32,
        s: [String; h]
    }
    let mut coodinates = vec![];
    for (i, _s) in s.iter().enumerate(){
        for (j, __s) in _s.chars().enumerate(){
            if is_o(__s){
                coodinates.push([i, j]);
            }
        }
    }
    println!("{}", ((coodinates[0][0] as i32)-(coodinates[1][0] as i32)).abs()+((coodinates[0][1] as i32)-(coodinates[1][1] as i32)).abs());
}
