use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        a: [[i32; 2]; 2]
    }
    println!("{}", a[r-1][c-1])
}


//pub fn abc_255_b() {
//    input! {
//        n: usize,
//        k: usize,
//        a: [usize; n],
//    }
//    let xy: [[i64; 2]; n];
//}
