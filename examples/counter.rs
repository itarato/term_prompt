use std::{thread, time::Duration};

use term_prompt::counter::Counter;

fn main() {
    let mut counter = Counter::new("Animals".to_string());

    for i in 0..=123 {
        thread::sleep(Duration::from_millis(20));
        counter.inc(i);
    }

    counter.complete();
}
