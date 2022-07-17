/*
 * FileName:     abc_231
 * CreatedDate:  2022-07-17 14:33:56 +0900
 * LastModified: 2022-07-17 15:20:48 +0900
 */

use proconio::input;
use std::collections::HashMap;


pub fn abc_231_b() {
    input!{
        n: usize,
        s: [String; n],
    }
    let mut map = HashMap::new();
//    let mut map:HashMap<String, i32> = HashMap::new();
    for _s in s{
        *map.entry(_s).or_insert(0)  += 1;
    }
    let mut temp_idx:i32 = 0;
    let mut ans:String = (&"").to_string();
    for (k, v) in &map{
        if (temp_idx<*v){
            temp_idx = *v;
            ans = (&k).to_string();
        }
    }
    println!("{}", ans);
}
