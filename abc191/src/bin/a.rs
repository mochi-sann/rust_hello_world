// use proconio::input;
// use super::proconio::input;

fn main() {
    // input! {
    //     v: i32,
    //     t: i32,
    //     s: i32,
    //     d: i32,
    // }
    let mut v:i32 = new(); // 入力した値を入れる変数
    let mut t:i32 = new(); // 入力した値を入れる変数
    let mut s:i32 = new(); // 入力した値を入れる変数
    let mut d:i32 = new(); // 入力した値を入れる変数

    // let v = get_input();
    // let t = get_input();
    // let s = get_input();
    // let d = get_input();

    let second = d / v;
    if second <= t || s <= second {
        println!("Yes")
    } else {
        println!("No")
    }
}


// fn get_input() -> i32 {
//     let mut number = String::new();
//     io::stdin().read_line(&mut number).ok();
//     return number.trim().parse().ok().unwrap();
// }