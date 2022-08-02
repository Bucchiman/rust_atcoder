/*
 * FileName:     abc_138
 * CreatedDate:  2022-07-04 08:05:31 +0900
 * LastModified: 2022-08-02 22:11:34 +0900
 */

use proconio::input;


pub fn abc_138_a(){
    input!{
        n: usize,
        mut a: [f32; n],
    }
    let mut ans = 0.0;
    for _a in a{
        ans = ans + 1.0/_a;
    }
    println!("{}", 1.0/ans);
}

pub fn abc_138_c() {
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
