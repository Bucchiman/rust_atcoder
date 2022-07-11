use proconio::input;

fn main() {
    input!{
        n: i32,
        st: [[String; 2]; n],
    }
    let mut set = std::collections::HashSet::new();
    for _st in st{
        if set.contains(&_st){
            println!("Yes");
            return;
        }
        set.insert(_st);
    }
    println!("No");
}
