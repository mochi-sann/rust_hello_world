use proconio::input;


fn main() {
    input! {
        k: i32,
        a: i32,
        b: i32,
    }

    let index = a % k;
    if a + index <= b {
        println!("OK");
    } else if a + index {

    }
}
