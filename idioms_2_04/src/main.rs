// Default trait

use std::{path::PathBuf, time::Duration};

#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
    output: Option<PathBuf>,
    search_path: Vec<PathBuf>,
    timeout: Duration,
    check: bool,
}

fn main() {
    let mut conf = MyConfiguration::default();
    conf.check = true;
    println!("conf = {:#?}", conf);

    let conf1 = MyConfiguration {
        check: true,
        ..Default::default()
    };
    println!("conf1 = {:#?}", conf1);
}
