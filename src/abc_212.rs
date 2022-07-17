use proconio::input;
use std::char;
 
pub fn abc_212_a() {
    input! {
        a: usize,
        b: usize,
    }
    if a == 0 {
        println!("Silver");
    }
    else if b == 0 {
        println!("Gold");
    }
    else {
        println!("Alloy");
    }
}

pub fn abc_212_b() {
    input! {
        x: String,
    }
    let mut i:i32 = 0;
    let mut pre:i32 = 0;
    let mut flag:i32 = 0;
    for x_char in x.as_str().chars() {
        if (i==0){
            i += 1;
            pre = (x_char.to_string()).parse::<i32>().unwrap();
            continue;
        }
        if (pre != (x_char.to_string()).parse::<i32>().unwrap()){
            flag = 1;
            break;
        }
        i += 1;
    }
    if (flag == 0){
        println!("Weak");
        return;
    }
    i = 0;
    flag = 0;
    for x_char in x.as_str().chars() {
        if (i==0){
            i += 1;
            pre = (x_char.to_string()).parse::<i32>().unwrap();
            continue;
        }
        if ((pre+1)%10 != (x_char.to_string()).parse::<i32>().unwrap()){
            flag = 1;
            println!("Strong");
            return;
        }
        pre = (x_char.to_string()).parse::<i32>().unwrap();
        i += 1;
    }
    println!("Weak");
}
