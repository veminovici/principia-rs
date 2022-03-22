// Constructor
// https://rust-unofficial.github.io/patterns/idioms/ctor.html

// use an associated function `new` to create an object.


pub struct Second {
    value: u64,
}

impl Second {
    pub fn new(value: u64) -> Self {
        Self {
            value
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}

fn main() {
    println!("Hello, world!");
}
