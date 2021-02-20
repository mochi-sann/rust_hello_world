// use proconio::input;
// use super::proconio::input;
// use proconio::input;

fn main() {
    // input! {
    //     v: i32,
    //     t: i32,
    //     s: i32,
    //     d: i32,
    // }
    // input! {
    //     s: String,
    // }
    let s = "dIfFiCuLt";
    let mut count: i32 = 0;

    for c in s.chars() {
        count += 1;
        print!("{} {} ", c, count);
        if ((count % 2 == 0) && c.is_uppercase()) {
            println!("No");
            return;
        } else if ((count % 2 == 1) && !c.is_uppercase()) {
            println!("No");
            return;
        } else if {
            println!("Yes");
            return;
        }
    }
}
