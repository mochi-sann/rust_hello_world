// use proconio::input;
// use super::proconio::input;
use proconio::input;
use std::convert::TryInto;
fn main() {
    // input! {
    //     v: i32,
    //     t: i32,
    //     s: i32,
    //     d: i32,
    // }
    input! {
        s: String,
    }
    // let s = "dIFFiCuLt";
    let mut count: i32 = 0;

    for c in s.chars() {
        count += 1;
        // print!("{} {} ", c, count);
        if (count % 2 == 0) && !c.is_uppercase() {
            println!("No");
            return;
        } else if (count % 2 == 1) && c.is_uppercase() {
            println!("No");
            return;
        } else if s.len() == count.try_into().unwrap() {
            // println!("Yes");
            println!("Yes");
            return;
        } 
    }
}
