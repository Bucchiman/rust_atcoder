use proconio::input;

fn main() {
    input!{
        a: [i32; 10],
    }
    let mut i: i32 = 0;
    let mut idx: i32 = 0;
    while (i < 3){
        idx = a[idx as usize];
        i += 1;
    }
    println!("{}", idx);
}
