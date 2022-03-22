// Rust does not have finally, use the destructor

fn bar() -> Result<(), ()> {
    struct Foo;

    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }

    let _exit = Foo;

    Ok(())
}

fn main() {
    bar().unwrap();
    println!("Hello, world!");
}
