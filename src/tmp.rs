use proconio::input;

fn main() {
    input!{
        s: String,
    }
    for (i, s_) in s.chars().enumerate(){
        if i % 2 == 0 && (s_ == 'R' || s_ == 'U' || s_ == 'D'){

        }
        else if i % 2 == 1 && (s_ == 'L' || s_ == 'U' || s_ == 'D'){

        }
        else{
            println!("No");
            return
        }
    }
    println!("Yes");

}

