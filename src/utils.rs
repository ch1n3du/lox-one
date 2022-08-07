use std::fs::File;
use std::io::Read;

use std::fmt::Display;

pub fn read_file(path: &str) -> String {
    let mut file = File::open(path)
        .unwrap_or_else(|_| panic!("Error opening file in 'assert_execution_of_file'"));

    let mut src = String::new();
    file.read_to_string(&mut src)
        .unwrap_or_else(|_| panic!("Error reading file in 'assert_execution_of_file'"));

    src
}

pub fn log_items<T: Display>(log_title: &str, items: &Vec<T>) {
    if items.len() != 0 {
        let sep = "======================================================================================";

        println!("\n{}", sep);
        println!("{}:\n{}\n", log_title, sep);

        for (i, item) in items.iter().enumerate() {
            println!("{} -> Error: {}", i, item);
        }

        // println!("\nFinished reporting errors.");
        println!("\n{}\n", sep);
    }
    // panic!()
}
