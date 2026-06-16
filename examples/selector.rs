use term_prompt::Selector;

fn main() {
    let selected = Selector::run(
        "Pick an animal:".to_string(),
        vec![
            "Rabbit".to_string(),
            "Fennec".to_string(),
            "Seal".to_string(),
            "Tiger".to_string(),
        ],
        Some(1),
    );

    println!("Selection: {:?}", selected);
}
