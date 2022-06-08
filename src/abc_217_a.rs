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
