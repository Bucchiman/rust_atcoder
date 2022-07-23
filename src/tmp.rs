use proconio::input;
use std::convert::TryInto;
use std::collections::HashMap;

fn main() {
    input!{
        n: i64,
        k: i64,
        c: [i64; n],
    }
    let mut map = HashMap::new();
    let mut ans:i64 = 0;
    let mut i:i64 = 0;
    let mut j:i64 = 0;
    for i in 0..(n-k+1) {
        if i == 0 {
            for j in i..k{
                *map.entry(c[j as usize]).or_insert(0)  += 1;
            }
        }
        else{
            *map.entry(c[i as usize]).or_insert(0) -= 1;
            if map.get(&c[i as usize]) <= Some(&0){
                map.remove(&c[i as usize]);
            }
            *map.entry(c[(i+k-1) as usize]).or_insert(0) += 1;
        }
        if ans < map.keys().len().try_into().unwrap(){
            ans = map.keys().len().try_into().unwrap();
        }
    }
    println!("{}", ans);
}
