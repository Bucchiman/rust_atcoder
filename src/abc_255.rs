/*
 * FileName:     abc_255
 * CreatedDate:  2022-06-14 18:19:02 +0900
 * LastModified: 2022-06-14 23:18:24 +0900
 */

use proconio::input;


pub fn abc_255_a() {
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
