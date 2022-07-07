/*
 * FileName:     abc_242
 * CreatedDate:  2022-07-07 22:48:58 +0900
 * LastModified: 2022-07-07 22:56:46 +0900
 */

use proconio::input;

pub fn abc_242_b(){
    input!{
        x: String,
    }
    let mut it: Vec<_> = x.chars().collect();
    let mut y = String::new();

    it.sort();

    for c in it {
        y.push(c);
    }
    let y: String = y.chars().into_iter().collect();

    println!("{}", y);
}
