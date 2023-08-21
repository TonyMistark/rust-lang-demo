// example 001
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// example 002
fn longest_s<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// example 003
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let s1 = String::from("long string is long");
    {
        let s2 = String::from("xyz");
        // example 001
        let result = longest(s1.as_str(), s2.as_str());
        println!("The longest string is {}", result);
        // example 002
        let result = longest_s(&s1, &s2);
        println!("The longest string is {}", result);

        // example 003
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split(".").next().expect("Could not finde a '.'");
        let i = ImportantExcerpt{
            part: first_sentence,
        };
        println!("{i:?}");
    }
}
