// use proconio::input;
// use super::proconio::input;
use proconio::input;

fn main() {
    // input! {
    //     v: i32,
    //     t: i32,
    //     s: i32,
    //     d: i32,
    // }
    input! {
        x: i32,
    }

    // let v = get_input();
    // let t = get_input();
    // let s = get_input();
    // let d = get_input();

    let result = x % 100;
    if result == 100 {
        println!("100");
    } else {
        println!("{}", 100 -result);
    }
}
