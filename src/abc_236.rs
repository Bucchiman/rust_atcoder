/*
 * FileName:     abc_236
 * CreatedDate:  2022-07-15 08:28:10 +0900
 * LastModified: 2022-07-15 18:03:01 +0900
 */

use proconio::input;


pub fn abc_236_a() {
    input!{
        s: String,
        a: usize,
        b: usize,
    }
    print!("{}{}{}{}{}\n", &s[0..(a-1)], &s[(b-1)..b], &s[a..(b-1)], &s[(a-1)..a], &s[b..s.chars().count()]);
}
