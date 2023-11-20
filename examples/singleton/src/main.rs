use lazy_static::lazy_static;
use std::sync::Mutex;

// single struct
#[derive(Debug)]
struct Singleton {
    count: i32
}

lazy_static!{
    static ref SINGLETON_INSTANCE: Mutex<Singleton> = Mutex::new(Singleton{count: 10});
}

fn get_instance() -> &'static Mutex<Singleton> {
    &SINGLETON_INSTANCE
}

fn main() {
    let singleton = get_instance().lock().unwrap();
    println!("{singleton:?}");
}
