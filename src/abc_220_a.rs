use proconio::input;

pub fn abc_220_a() {
    input! {
        A: usize,
        B: usize,
        C: usize
    }
    for n in A..B+1 {
        if n % C == 0 {
            println!("{}", n);
            return;
        }
    }
    println!("{}", -1)
}
