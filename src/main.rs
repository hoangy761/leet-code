use std::{sync::Arc, thread};

fn main() {
    let value = Arc::new(10);

    thread::spawn(move || {
        println!("{value}");
    });
}
