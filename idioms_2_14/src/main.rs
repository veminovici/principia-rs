// Temporary mutability
// https://rust-unofficial.github.io/patterns/idioms/temporary-mutability.html

// nested block

// let data = {
//     let mut data = get_vec();
//     data.sort();
//     data
// }

// Here data is immutable

// using variable rebinding

// let mut data = get)vec();
// data.sort();
// let data = data;

// Here data is immutable


fn main() {
    println!("Hello, world!");
}
