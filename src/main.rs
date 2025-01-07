use std::{io, io::BufRead};

use crossbeam::thread;

fn main() {
    thread::scope(|scope| {
        // Input thread
        scope.spawn(|_| {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(input) => println!("{}", input),
                    Err(_) => {}
                }
            }
            println!("Waiting for input");
        });

        // Worker thread
        scope.spawn(|_| {
            println!("Processing...");
        });

        // Output thread
        scope.spawn(|_| {
            println!("Output processor...");
        });
    })
    .unwrap();
}
