use proconio::input;

fn main() {
    input!{
        l: usize,
        r: usize,
        s: String,
    }
    println!("{}{}{}", &s[0..(l-1)], &s[(l-1)..r].chars().rev().collect::<String>(), &s[r..s.len()]);
}
