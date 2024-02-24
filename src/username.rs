use axum_session::SessionSurrealPool;
use surrealdb::engine::any::Any;

pub fn get_username(session: &axum_session::Session<SessionSurrealPool<Any>>) -> String {
    let mut username: String = session.get("username").unwrap_or("".to_string());
    if username.trim().is_empty() {
        username = random_username();
        session.set("username", username);
    }

    let username = session
        .get::<String>("username")
        .expect("Error getting username");
    return username;
}

pub fn random_username() -> String {
    let name = names::Generator::default().next().unwrap_or_default();
    return capitalize_words(name.as_str());
}

fn capitalize_words(input: &str) -> String {
    let mut result = String::new();

    for word in input.split('-') {
        if !result.is_empty() {
            result.push(' '); // Add a space between words
        }

        let (first, rest) = word.split_at(1);
        let capitalized_word = first.to_uppercase() + rest;
        result.push_str(&capitalized_word);
    }

    result
}
