use proconio::input;

use std::collections::HashMap;


pub fn rec(a: i64, k: i64) -> i64{
    let mut ans: i64 = 1;
    for i in 1..(k+1){
        ans *= a;
    }
    return ans;
}

fn main() {
    input!{
        k: i64,
        a: String,
        b: String,
    }
    let mut a10: i64 = 0;
    let mut b10: i64 = 0;
    for (i, _a) in a.chars().enumerate(){
        a10 += ((_a as i64)-48)*rec(k, (a.len() as i64)-((i as i64)+1));
        //println!("{}", rec(k, (a.len() as i64)-((i as i64)+1)));
    }
    for (i, _b) in b.chars().enumerate(){
        b10 += ((_b as i64)-48)*rec(k, (b.len() as i64)-((i as i64)+1));
    }

    println!("{}", a10*b10);
}
