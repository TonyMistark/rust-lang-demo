// cache u32, but can not cache str
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(query: T) -> Cacher<T> {
        Cacher { query: query, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => {
                println!("Exists cache, return self.value directly");
                v
            },
            None => {
                println!("Does exists cache, query and cache it");
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
    
}

struct AdvanceCacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy
{
    query: T,
    value: Option<E>,
}

impl<T, E> AdvanceCacher<T, E>
where
    T: Fn(E) -> E,
    E: Copy
{
    fn new(query: T) -> AdvanceCacher<T, E> {
        AdvanceCacher { query, value: None }
    }

    fn value(&mut self, arg: E) -> E {
        match self.value {
            Some(v) => {
                println!("Exists cache, return self.value directly");
                v
            },
            None => {
                println!("Does exists cache, query and cache it");
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[derive(Debug)]
struct MyString {
    content: String
}

impl MyString {
}


// 三种Fn特征
// 闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：
// 转移所有权、可变借用、不可变借用，
// 因此相应的 Fn 特征也有三种：
// 1.FnOnce，该类型的闭包会拿走被捕获变量的所有权。Once 顾名思义，说明该闭包只能运行一次：

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{

    println!("{}", func(3));
    // fn once
    // println!("{}", func(4));
}

fn fn_once_call_for_test() {
    let x = vec![1, 2, 3];
    fn_once(|z| {z == x.len()});
}

// 这里面有一个很重要的提示，因为 F 没有实现 Copy 特征，
// 所以会报错，那么我们添加一个约束，试试实现了 Copy 的闭包：
fn fn_once001<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("first time: {}", func(3));
    println!("second time: {}", func(4));
}

fn fn_once001_call_for_test() {
    let x = vec![1, 2, 3];
    fn_once001(|z| {z == x.len()});
}

// 如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 move 关键字，
// 这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程。
fn move_demo()
{
    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        println!("Here's a vector: {:?} again", v);
    });
    handle.join().unwrap();
    // can't use, borrowed before
    //println!("Here's a vector: {:?} later", v);

}

// 2.FnMut, 它以可变借用的方式捕获了环境中的值，因此可以修改该值：

fn fn_mut(){
    let mut s = String::new();
    // - calling `update_string` requires mutable binding due to mutable borrow of `s`
    // let update_string = |str| s.push_str(str);

    // let `mut` 想要在闭包内部捕获可变借用，需要把该闭包声明为可变类型，
    // 也就是 update_string 要修改为 mut update_string
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}", s);
}

// 这种写法有点反直觉，相比起来前面的 move 更符合使用和阅读习惯。
// 但是如果你忽略 update_string 的类型，仅仅把它当成一个普通变量，
// 那么这种声明就比较合理了。

fn fn_mut_another() {
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    println!("{:?}", s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello");
    f("hello");
}

// 3.Fn 特征, 它以不可变借用的方式捕获环境中的值 
// 让我们把上面的代码中 exec 的 F 泛型参数类型修改为 Fn(&'a str)：

fn fn_trait() {
    // let mut s = String::new();
    // let update_string = |str| s.push_str(str);
    let s = "hello, ".to_string();

    let update_string = |str| println!("{}, {}", s, str);

    exec01(update_string);
    println!("{:?} end", s);
}

fn exec01<'a, F: Fn(String)>(f: F){
    f("world".to_string())
}

// move and Fn
// 在上面，我们讲到了 move 关键字对于 FnOnce 特征的重要性，
// 但是实际上使用了 move 的闭包依然可能实现了 Fn 或 FnMut 特征。

// 因为，一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，
// 而不是取决于闭包如何捕获它们。move 本身强调的就是后者，闭包如何捕获变量：

fn move_and_fn_01() {
    let s = String::new();
    let update_string = move || println!("move_and_fn_01: {}", s);
    exec_01(update_string);
    let s2 = String::new();
    let update_string2 = move || println!("move_and_fn_01: {}", s2);
    exec_02(update_string2);
}

fn exec_01<F:FnOnce()>(f: F) {
    f()
}

fn exec_02<F: Fn()>(f: F) {
    f()
}

// 三种Fn的关系
// 1. 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
// 2. 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
// 3. 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征
// 用一段代码来简单诠释上述规则：

fn three_fn_relationships() {
    let s = String::new();
    let update_string = || println!("three_fn_relationships:{}", s);

    execute1(update_string);
    execute2(update_string);
    execute3(update_string);
}

fn execute1<F: FnOnce()>(f: F) {
    f()
}

fn execute2<F: FnMut()>(mut f: F) {
    f()
}

fn execute3<F: Fn()>(f: F) {
    f()
}

fn three_fn_relationships001() {
    let mut s = String::new();
    s.push_str("Hello, ");

    let update_string = |str| { s.push_str(str); s };
    exec_001(update_string);
}

fn exec_001<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
    let result_s = f("world");
    println!("result_s: {result_s}");
}

fn main() {
    let mut c = Cacher::new(|x| x + 1);
    println!("value: {}", c.value(2));
    println!("value: {}", c.value(3));

    let mut ac = AdvanceCacher::new(|x| x + 1);
    println!("value: {}", ac.value(2));
    println!("value: {}", ac.value(3));

    let mut ac2 = AdvanceCacher::new(|a| a);
    println!("value: {}", ac2.value("ice"));
    println!("value: {}", ac2.value("ice"));

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;

    assert!(equal_to_x(y));

    fn_once_call_for_test();
    fn_once001_call_for_test();

    move_demo();

    fn_mut();
    fn_mut_another();

    fn_trait();

    move_and_fn_01();

    three_fn_relationships();
    three_fn_relationships001();

}
