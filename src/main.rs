mod storage;
fn _first_title(titles: &[String]) -> Option<&str> {
    let first = titles.first()?;
    Some(&first[..])
}

fn _load_quest_titles(path: &str) -> Result<Vec<String>, std::io::Error> {
    let titles = std::fs::read_to_string(path)?;

    let titles_vec = titles
        .lines()
        .map(|title| title.trim())
        .filter(|title| !title.is_empty())
        .map(|title| title.to_string())
        .collect();
    Ok(titles_vec)
}

fn main() {
    // if let Ok(current_dir) = std::env::current_dir() {
    //     println!("Path: {:?}", current_dir)
    // }
    // let path = "data/quests.txt";
    // match load_quest_titles(path) {
    //     Ok(titles) => {
    //         for title in &titles {
    //             println!("{}", title)
    //         }
    //         println!();
    //         println!("First title .....");
    //         let first = first_title(&titles).unwrap_or("Nothing Exists....");
    //         println!("{}", first)
    //     }
    //     Err(error) => println!("Error: {}", error),
    // }

    let titles = vec![
        "Chat Gpt".to_string(),
        "Chad Gpt".to_string(),
        "Rust Gpt".to_string(),
        "Gpt Chat".to_string(),
    ];

    match std::env::args().nth(1) {
        Some(path) => {
            println!("Writing.... on path: '{}'", path);
            let result = storage::save_quest_titles(&path, &titles);
            let message = match result {
                Ok(()) => "Sucess.",
                Err(er) => &er.to_string(),
            };
            println!("Result: {} ", message);
        }
        None => println!("Couldn't recive a file path to write."),
    };
}
