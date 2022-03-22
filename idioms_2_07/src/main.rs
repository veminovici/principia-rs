// mem::{take(_), replace(_)} to keep owned values in changed enums
// https://rust-unofficial.github.io/patterns/idioms/mem-replace.html

// An enum with two variants.

use std::mem;

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String }
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0} = e {
        *e = MyEnum::B {name: mem::take(name)}
    }
}

enum MyEnum1 {
    A { name: String },
    B { name: String },
    C,
    D
}

fn swizzle(e: &mut MyEnum1) {
    use MyEnum1::*;

    *e = match e {
        A { name } => B { name: mem::take(name)},
        B { name } => A { name: mem::take(name)},
        C => D,
        D => C,
    }
}

fn main() {
    println!("Hello, world!");
}
