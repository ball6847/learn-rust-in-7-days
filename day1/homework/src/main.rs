use std::env::args;

fn main() {
    match get_name() {
        Ok(name) => println!("Hello, {}", name),
        Err(m) => println!("Err: {}", m),
    }
}

fn get_name() -> Result<String, String> {
    let name = args().nth(1).unwrap_or("".to_string());

    if name == "" {
        return Err("Name should be provided in command line argument".to_string());
    }

    if !name.to_string().to_lowercase().starts_with('w') {
        return Err("Name should start with W".to_string());
    }

    Ok(name)
}
