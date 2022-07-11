/*
 * FileName:     abc_216
 * CreatedDate:  2022-07-09 09:05:40 +0900
 * LastModified: 2022-07-12 08:44:11 +0900
 */

use proconio::input;


pub fn abc_216_a() {
    input! {
        number: String
    }
    let v: Vec<&str> = number.split('.').collect();
    let Y: i64 = v[1].parse().unwrap();
    if 0 <= Y && Y <= 2 {
        println!("{}", v[0].to_string()+"-")
    }
    else if 2 < Y && Y <= 6 {
        println!("{}", v[0].to_string())
    }
    else {
        println!("{}", v[0].to_string()+"+")
    }
}

pub fn abc_216_b() {
    input!{
        n: i32,
        st: [[String; 2]; n],
    }
    let mut set = std::collections::HashSet::new();
    for _st in st{
        if set.contains(&_st){
            println!("Yes");
            return;
        }
        set.insert(_st);
    }
    println!("No");
}
