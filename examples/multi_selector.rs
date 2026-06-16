use std::collections::HashSet;

use term_prompt::multi_selector::MultiSelector;

fn main() {
    println!("Example:\n");

    let selections = MultiSelector::run(
        "Pick an animal:".to_string(),
        vec![
            "Rabbit".to_string(),
            "Fennec".to_string(),
            "Seal".to_string(),
            "Tiger".to_string(),
        ],
        HashSet::from([1]),
    );

    println!("Multi selection: {:?}", selections);
}
