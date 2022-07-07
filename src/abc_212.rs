use proconio::input;
 
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
    let i: i64 = 0;
    for i in x.as_str().chars() {
        println!("{}", i);
    }
    
}
