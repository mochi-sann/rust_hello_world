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
        a: f64,
        b: f64,
    }

    let sa = a - b;
    let result: f64 = (sa * 100.0) / a ;

    println!("{}", result);
}
