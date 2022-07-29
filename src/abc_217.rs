use std::any:: type_name;
use proconio:: input;
fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

pub fn abc_217_a() {
    input! {
        S: String,
        T: String
    }
    if S < T {
        println!("Yes");
    }
    else {
        println!("No");
    }
}



pub fn abc_217_b() {
    input!{
        n: usize,
        p: [usize; n],
    }
    let mut q = vec![0; n];
    for (i, _p) in p.iter().enumerate(){
        q[(_p-1) as usize] = i+1;
    }
    for _q in q.iter(){
        print!("{} ", _q);
    }
    println!("");
}
