/*
 * FileName:     abc_221
 * CreatedDate:  2022-06-10 22:10:39 +0900
 * LastModified: 2022-08-02 21:28:40 +0900
 */

use proconio::input;


pub fn abc_221_a() {
    input! {
        a: u32,
        b: u32,
    }
    println!("{}", 32_i32.pow(a-b))
}


pub fn abc_221_b() {
    input!{
        s: String,
        t: String,
    }
    if s == t{
        println!("Yes");
    }
    let l = s.len();
    let s = s.chars().into_iter().collect::<Vec<char>>();
    let t = t.chars().into_iter().collect::<Vec<char>>();
    let mut ok = false;
    let mut count = 0;
    for i in 0..l {
        let si = s[i];
        let ti = t[i];
        if si != ti {
            count += 1;
            if i < l-1 {
                let si1 = s[i+1];
                let ti1 = t[i+1];
                if si == ti1 && ti == si1 {
                    ok = true;
                }
            }
        }
    }
    if count == 2 && ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
