// 显式迭代
fn f1() {
    let arr = [1,2 ,3];

    for v in arr.into_iter() {
        println!("{v}");
    }

}
// 隐式迭代
fn f() {
    let arr = [1,2 ,3];

    for v in arr {
        println!("{v}");
    }

}
fn main() {
    f();
    f1();
}
