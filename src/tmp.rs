use proconio::input;

fn main() {
    input!{
        s: String,
        a: usize,
        b: usize,
    }
    print!("{}{}{}{}{}\n", &s[0..(a-1)], &s[(b-1)..b], &s[a..(b-1)], &s[(a-1)..a], &s[b..s.chars().count()]);
}
