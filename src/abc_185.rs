/*
 * FileName:     abc_185
 * CreatedDate:  2022-06-21 21:23:48 +0900
 * LastModified: 2022-06-21 21:37:17 +0900
 */

use proconio::input;


pub fn abc_185_a() {
    input!{
        a: [usize; 4],
    }
/*    for i in 0..4{
        if(ans > a[i]){
            ans = a[i];
        }
    }*/
    println!("{}", a.iter().min().unwrap());
}
