fn main() {
    let s = String::from("Hello World กกก");

    // this is size of bytes, not char
    println!("S size in bytes = {}", s.len());

    // this count total characters in string
    println!("S chars count = {}", s.chars().count());

    // loop through each characters in string
    for c in s.chars() {
        println!("{}", c);
    }

    // loop through each characters in string along with its index
    for (i, c) in s.chars().enumerate() {
        println!("{} = {}", i, c);
    }

    // loop through each characters in string along with its byte offset
    for (i, c) in s.char_indices() {
        println!("{} = {}", i, c);
    }

    let c_l = count_l(&s);

    println!("total l char in string = {}", c_l);
}

fn count_l(s: &str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res += 1;
        }
    }
    res
}
