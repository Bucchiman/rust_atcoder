use proconio::input;

fn main() {
    input!{
        k: usize,
        a: usize,
        b: usize,
    }
    if a/k*k >= a || a/k*k+k<=b{
        println!("OK");
    }
    else{
        println!("NG");
    }
}
