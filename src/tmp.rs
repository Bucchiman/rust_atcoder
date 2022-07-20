use proconio::input;

fn main() {
    input! {
        n: i32,
        s: String,
    }
    for (i, _s) in s.chars().enumerate(){
        if (i == (n-1) as usize && _s == 'o'){
            println!("Yes");
        }
        else if (i == (n-1) as usize && _s == 'x'){
            println!("No");
        }
    }
}
