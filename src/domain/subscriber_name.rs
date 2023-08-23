use unicode_segmentation::UnicodeSegmentation;

const FORBIDDEN_CHARS: [char; 13] = [
    '/', '(', ')', ';', '"', '<', '>', '\\', '{', '}', ',', '.', ':',
];

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(name: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_too_long = name.graphemes(true).count() > 256;

        let contains_forbidden_char = name.chars().any(|g| FORBIDDEN_CHARS.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_char {
            return Err(format!("{} is not a valid subscriber name", name));
        }

        Ok(Self(name))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::subscriber_name::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ü".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_257_grapheme_long_name_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in super::FORBIDDEN_CHARS {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }
}
