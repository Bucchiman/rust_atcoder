/*
 * FileName:     abc_140
 * CreatedDate:  2022-06-17 18:33:00 +0900
 * LastModified: 2022-06-20 18:31:11 +0900
 */

use proconio::input;


pub fn abc_140_a(){
    input!{
        n: i32,
    }
    println!("{}", n.pow(3));
}

pub fn abc_140_b(){
    input!{
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n-1]
    }
    let mut pre_cuisine: usize = 0;
    let mut ans: usize = 0;
    for (i, &a_idx) in a.iter().enumerate() {
        if i == 0 {
            pre_cuisine = a_idx as usize;
            ans += b[(a_idx as usize)-1];
            continue;
        }
        if pre_cuisine + 1 == (a_idx as usize) {
            ans = ans + (c[(pre_cuisine as usize)-1]+b[(a_idx as usize)-1]);
        }
        else{
            ans += b[(a_idx as usize)-1];
        }
        pre_cuisine = a_idx as usize;
    }
    println!("{}", ans);
}
