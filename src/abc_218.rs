/*
 * FileName:     abc_218
 * CreatedDate:  2022-06-08 17:07:33 +0900
 * LastModified: 2022-07-21 00:05:20 +0900
 */

use proconio:: input;
use std::collections::HashMap;



pub fn abc_218_a() {
    input! {
        n: i32,
        s: String,
    }
    for (i, _s) in s.chars().enumerate(){
        if (i == (n-1) as usize && _s == 'o'){
            println!("Yes");
        }
        else if (i == (n-1) as usize && _s == 'x'){
            println!("No");
        }
    }
}

pub fn abc_218_b() {
    let mut map = HashMap::new();
    for i in 1..27{
        map.insert(i, ((i as u8)+96) as char);
        //println!("{}", ((i as u8)+96) as char);
    }
    input!{
        p: [i32; 26],
    }
    for _p in p{
        print!("{}", map[&_p]);
    }
    print!("\n");
}
