use std::collections::HashMap;
use std::env::args;

fn main() {
    working_with_option();
    working_with_result();
}

fn working_with_option() {
    let mut map = HashMap::new();

    map.insert(3, "Hello");
    map.insert(5, "World");

    // map.get() returns Option, we can extract it using match or unwrwap()

    // -----------------------------
    // using match

    // let r = match map.get(&3) {
    //     Some(v) => v,
    //     _ => "Nothing",
    // };

    // -----------------------------
    // using unwrap, if the value is found, return it, otherwise panick
    // unwrap() is a method provided by Option type, allow us to quickly extract the value from Option

    // let r = map.get(&3).unwrap();

    // -----------------------------
    // using unwrap_or() allows us to provide fallback value if no value in Option without panicking the progrma
    // note that, we use &"Nothing" as map.get() returns &str and so should the fallback

    let r = map.get(&3).unwrap_or(&"Nothing");

    // this should print "Hello"
    println!("{}", r);
}

fn working_with_result() {
    // handle result using match
    match get_args(3) {
        Ok(v) => println!("OK - {}", v),
        Err(e) => println!("Err - {}", e),
    }
}

fn get_args(n: usize) -> Result<String, String> {
    for (i, a) in args().enumerate() {
        if i == n {
            return Ok(a);
        }
    }
    Err("Not enough args".to_string())
}
