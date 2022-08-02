use proconio::input;

use std::collections::HashMap;

fn main() {
    input!{
        n: i32,
        mut v: [i32; n],
    }
    v.sort();
    let mut ans:f32 = 0.0;
    for i in 0..n{
        if i == 0{
            ans = v[i as usize] as f32;
        }
        else{
            ans = ((v[i as usize] as f32)+ans)/2.0;
        }
    }
    println!("{}", ans);
}
