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

}
