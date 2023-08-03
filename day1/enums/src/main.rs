#[derive(Debug)]
pub struct Bed {
    size: i32,
    count: u32,
}

// enum can hold a value or struct
#[derive(Debug)]
pub enum Room {
    Kitchen(i32),
    Bedroom(Bed),
    Bathroom(i32, String), // can hold as many values
    Lounge,
}

fn main() {
    let lounge = Room::Lounge;
    println!("Hello from the {:?}", lounge);

    // kitchen hold the value we need to pass it
    let t = Room::Kitchen(4);
    println!("Hello from the {:?}", t);

    match &t {
        Room::Kitchen(n) => println!("The room is a kitchen with {} rooms", n),
        _other => println!("{:?}", _other),
    }

    let v = match &t {
        Room::Kitchen(n) => *n, // * is required as we match against the reference of t, where n will be become &i32 instead of i32, so we need to deref the value using * to make it i32
        _other => 0,
    };

    println!("{}", v);

    let b = Room::Bathroom(5, "big".to_string());

    // alternatively, we can match using if let instead
    if let Room::Bathroom(n, s) = b {
        println!("It's a {} bathroom with {} stuffs", s, n)
    }
}
