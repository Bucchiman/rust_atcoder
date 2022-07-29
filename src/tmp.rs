use proconio::input;

use std::collections::HashMap;

fn main() {
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
