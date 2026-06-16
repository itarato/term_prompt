# Rust Terminal Prompt Toolkit

Example:

![Screenshot](./misc/selection.png)

```rust
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
```

![Screenshot](./misc/multi_selection.png)

```rust
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
```
