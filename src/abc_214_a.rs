use proconio::input;


pub fn abc_214_a(){
    input! {
        N: usize,
    }
    if 1 <= N && N <= 125 {
        println!("4");
    }
    else if 126 <= N && N <= 211 {
        println!("5");
    }
    else {
        println!("6");
    }
}
