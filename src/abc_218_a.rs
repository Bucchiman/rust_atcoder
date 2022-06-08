/*
 * FileName:     abc_218_a
 * CreatedDate:  2022-06-08 17:07:33 +0900
 * LastModified: 2022-06-08 17:18:26 +0900
 */

use proconio:: input;


pub fn abc_218_a() {
    input! {
        N: usize,
        S: String
    }
    for s in 0..S.len() {
        if s == N {
            println!("{}", S[s]);
        }
    }
}
