/*
 * FileName:     abc_214
 * CreatedDate:  2022-07-29 22:19:21 +0900
 * LastModified: 2022-07-29 23:27:11 +0900
 */

use proconio::input;



pub fn abc_214_a(){
    input! {
        N: usize,
    }
    if 1 <= N && N <= 125 {
        println!("4");
    }
    else if 126 <= N && N <= 211 {
        println!("5");
    }
    else {
        println!("6");
    }
}

pub fn abc_214_c() {
    input!{
        n: i64,
        s: [i64; n],
        t: [i64; n]
    }
    let mut i: i64 = 0;
    let mut time: i64 = 0;
    while i<n {
        if i == 0{
            println!("{}", t[i as usize]);
            time = t[0];
            i += 1;
            continue;
        }
        if time+s[(i-1) as usize] < t[i as usize]{
            time += s[(i-1) as usize];
            println!("{}", time);
        }
        else{
            println!("{}", t[i as usize]);
            time = t[i as usize];
        }
        i += 1;
    }
}
