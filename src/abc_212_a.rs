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
