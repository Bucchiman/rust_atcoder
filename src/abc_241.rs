/*
 * FileName:     abc_241
 * CreatedDate:  2022-07-15 08:12:24 +0900
 * LastModified: 2022-07-19 23:48:49 +0900
 */

use proconio::input;


pub fn abc_241_a() {
    input!{
        a: [i32; 10],
    }
    let mut i: i32 = 0;
    let mut idx: i32 = 0;
    while (i < 3){
        idx = a[idx as usize];
        i += 1;
    }
    println!("{}", idx);
}


pub fn abc_241_b() {
    input!{
        n: usize,
        m: usize,
        a: [i32; n],
        b: [i32; m],
    }
    let mut buf_idx = Vec::new();
    for (i, _b) in b.iter().enumerate(){
        for (j, _a) in a.iter().enumerate(){
            if (_b == _a && !buf_idx.iter().any(|&e| e == j)){
                buf_idx.push(j);
                break;
            }
        }
    }
    if (buf_idx.len() == m){
        println!("Yes");
    }
    else{
        println!("No");
    }
}
