pub fn save_quest_titles(path: &str, titles: &[String]) -> Result<(), std::io::Error> {
    let joined = format_quest_titles(titles);
    std::fs::write(path, joined)?;
    Ok(())
}

fn format_quest_titles(titles: &[String]) -> String {
    titles.join("\n")
}

#[cfg(test)]
mod tests {

    use super::format_quest_titles;

    #[test]
    fn test_format() {
        let titles = vec!["Chat".to_string(), "Gpt".to_string()];
        let result = format_quest_titles(&titles);

        assert_eq!(result, "Chat\nGpt".to_string())
    }
}
