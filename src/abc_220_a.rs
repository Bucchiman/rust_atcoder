use proconio::input;

pub fn abc_220_a() {
    input! {
        A: usize,
        B: usize,
        C: usize
    }
    let amari: usize = A % C;
    println!("{}", 2*C-amari);
}
