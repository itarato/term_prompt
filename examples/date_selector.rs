use term_prompt::date_selector::DateSelector;

fn main() {
    println!("Example:\n");

    let selection = DateSelector::run("Pick a date:".to_string(), None);

    println!("Selected date: {:?}", selection);
}
