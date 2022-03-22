// Collection are Smart Pointers
// https://rust-unofficial.github.io/patterns/idioms/deref.html

// Collections are smart pointers
// Implementing Deref for Vec, allows implicit dereferencing from &Vec<T> to &[T]

use std::{ops::Deref, marker::PhantomData};

struct Vec<T> {
    marker: PhantomData<T>,
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
