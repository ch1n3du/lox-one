use std::fs::File;
use std::io::Read;

use std::fmt::Display;

pub fn read_file(path: &str) -> String {
    let mut file = File::open(path).expect(&format!("\n\nError opening: {}\n", path));

    let mut src = String::new();
    file.read_to_string(&mut src)
        .expect(&format!("\n\nError reading: {}\n", path));

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
    panic!()
}
