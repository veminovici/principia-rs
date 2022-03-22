// use format! over push and push_str

fn say_hello(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() {
    println!("{}", say_hello("vlad"));
}
