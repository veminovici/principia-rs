// #[non_exhaustive] and private fields for extensibility
// https://rust-unofficial.github.io/patterns/idioms/priv-extend.html

mod a {
    #[non_exhaustive]
    pub struct S {
        pub foo: i32,
    }

    #[non_exhaustive]
    pub enum AdmitMoreVariants {
        VariantA,
        VariantB,
        #[non_exhaustive]
        VariantC { a: String }
    }
}

fn print_matched_variants(s: a::S) {
    let a::S { foo: _, ..} = s;

    let some_enum = a::AdmitMoreVariants::VariantA;
    match some_enum {
        a::AdmitMoreVariants::VariantA => println!("It is an A"),
        a::AdmitMoreVariants::VariantB => println!("it's a B"),
        a::AdmitMoreVariants::VariantC { a, .. } => println!("It's a C"),
        _ => println!("It's a new variant")
    }
}

fn main() {
    let s = a::S { foo: 10 };
    print_matched_variants(s);
}
