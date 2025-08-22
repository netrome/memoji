pub fn search(query: &str) -> Vec<&'static emojis::Emoji> {
    emojis::iter()
        .filter(|emoji| emoji.name().contains(query))
        .take(100)
        .collect()
}
