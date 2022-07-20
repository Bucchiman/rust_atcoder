use proconio:: input;


pub fn abc_219_a() {
    input! {
        X: usize,
    }
    if X < 40 {
        println!("{}", 40-X);
    }
    else if 40 <= X && X < 70 {
        println!("{}", 70-X);
    }
    else if 70 <= X && X < 90 {
        println!("{}", 90-X);
    }
    else {
        println!("expert");
    }
}


pub fn abc_219_b() {
    input!{
        s1: String,
        s2: String,
        s3: String,
        t: String,
    }
    let mut ans: String = "".to_string();
    for _t in t.chars(){
        if (_t.to_digit(10) == Some(1)){
            ans = ans + &s1;
        }
        else if (_t.to_digit(10) == Some(2)){
            ans = ans + &s2;
        }
        else{
            ans = ans + &s3;
        }
    }
    println!("{}", ans);
}
