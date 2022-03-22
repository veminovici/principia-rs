// Pass variables for closure
// https://rust-unofficial.github.io/patterns/idioms/pass-var-to-closure.html

use std::rc::Rc;

fn main() {
    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);


    let closure = {
        // num1 is moved
        let num2 = num2.clone(); // num2 is cloned
        let num3 = num3.as_ref();  // num3 is borrowed
        move || {
            *num1 + *num2 + *num3
        }
    };

    let res = closure();

    println!("Result: {}", res);
}
