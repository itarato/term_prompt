use std::{thread, time::Duration};

use term_prompt::loading_bar::LoadingBar;

fn main() {
    let loading_bar = LoadingBar::new(99);

    for i in 0..100 {
        thread::sleep(Duration::from_millis(20));
        loading_bar.set(i);
    }

    loading_bar.complete();
}
