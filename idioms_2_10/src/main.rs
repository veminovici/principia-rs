// Iteratig over an Option

fn test() {
    let turing = Some("Turing");
    let mut logicians = vec!["Curry", "Kleene", "Markov"];

    logicians.extend(turing);

    // equivalent to
    if let Some(turing_inner) = turing {
        logicians.push(turing_inner);
    }

    println!("Logicians: {:#?}", logicians);
}

fn test1() {
    let turing = Some("Turing");
    let logicians = vec!["Curry", "Kleene", "Markov"];

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{} is a logician", logician);
    }
}

fn main() {

    println!("Hello, world!");

    test();
    test1();
}
