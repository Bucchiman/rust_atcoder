/*
 * FileName:     abc_216_a
 * CreatedDate:  2022-06-08 16:44:55 +0900
 * LastModified: 2022-06-08 17:06:46 +0900
 */

use proconio:: input;

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
