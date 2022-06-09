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
