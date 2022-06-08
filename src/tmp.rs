use proconio::input;

fn main() {
    input! {
        number: String
    }
    let v: Vec<&str> = number.split('.').collect();
    let Y: i64 = v[1].parse().unwrap();
    if 0 <= Y && Y <= 2 {
        println!("{}", v[0].to_string()+"-")
    }
    else if 2 < Y && Y <= 6 {
        println!("{}", v[0].to_string())
    }
    else {
        println!("{}", v[0].to_string()+"+")
    }
}
