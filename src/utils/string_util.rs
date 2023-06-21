// Copyright (c) ZeroC, Inc.

/// Returns the indefinite article for the given word.
pub fn indefinite_article(s: &str) -> String {
    in_definite::get_a_or_an(s).to_lowercase()
}

// TODO expand this to be a general 'pluralize' method that isn't restricted to only the 'kind's of Slice elements.
///
pub fn pluralized_kind(s: &str) -> String {

}
