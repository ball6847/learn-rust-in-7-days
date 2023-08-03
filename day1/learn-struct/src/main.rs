#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    height: i32,
    shoe_size: i32,
}

impl User {
    // if we are doing readonly to the instance just pass &self (reference to instance)
    fn simple_string(&self) -> String {
        format!(
            "{} - {} - {}cm - shoe:{}",
            self.name, self.age, self.height, self.shoe_size
        )
    }

    // if we are going to change it, pass &mut self (mutable reference to instance)
    // this will cause you to declare a variable using mut keyword so we can change its value
    fn grow(&mut self, h: i32) {
        self.height += h;
    }

    // passing self with no & will move ownership of the insintace to this method and will make this able to called only once, after the instance will no longer accessible to the program
    fn die(self) {
        println!("Dead {} ", self.simple_string());
    }
}

fn main() {
    let mut u = User {
        name: "ball6847".to_string(),
        age: 38,
        height: 160,
        shoe_size: 42,
    };

    // {:?} allow us to debug the struct by printing it
    println!("User is {}", u.simple_string());

    u.grow(12);

    println!("User is {}", u.simple_string());
}
