use proconio::input;

fn main() {
    input!{
        s_x: f32,
        s_y: f32,
        g_x: f32,
        g_y: f32,
    }
    // y = (g_y+s_y)/(g_x-s_x)(x-g_x)+g_y
    println!("{}", -g_y*(g_x-s_x)/(g_y+s_y)+g_x);
}
