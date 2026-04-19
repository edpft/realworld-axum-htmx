use std::sync::LazyLock;

use regex::Regex;

pub struct Name(String);

impl TryFrom<String> for Name {
    type Error = String;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        if name.is_empty() {
            return Err(String::from("Empty"));
        }

        if name.len() > 64 {
            return Err(String::from("Too long"));
        }

        if is_url_safe(&name) {
            Ok(Self(name))
        } else {
            Err(String::from("Contains non URL safe characters"))
        }
    }
}

fn is_url_safe(string: &str) -> bool {
    !string.contains(|c: char| !c.is_ascii_alphanumeric() && !"-_.~".contains(c))
}


#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest::string::string_regex;

    use super::Name;

    #[test]
    fn a_short_valid_username_is_valid() {
        let name = String::from("a");

        assert!(Name::try_from(name).is_ok())
    }

    #[test]
    fn a_long_valid_username_is_valid() {
        let name = String::from("YaalnUp0R8ZYNRx99J29ER86LadaRFxhpzUki9oVNQBy1sE4EqNnp87CxDYeNj1p");

        assert!(Name::try_from(name).is_ok())
    }

    #[test]
    fn an_empty_string_is_invalid() {
        let name = String::from("");

        assert!(Name::try_from(name).is_err())
    }

    #[test]
    fn a_string_of_valid_characters_that_is_too_long_is_invalid() {
        let name =
            String::from("YaalnUp0R8ZYNRx99J29ER86LadaRFxhpzUki9oVNQBy1sE4EqNnp87CxDYeNj1pl");

        assert!(Name::try_from(name).is_err())
    }

    #[test]
    fn a_string_containing_a_non_url_safe_character_is_invalid() {
        let name = String::from("foo*");

        assert!(Name::try_from(name).is_err());
    }

    proptest! {
        #[test]
        fn any_username_of_between_1_and_64_valid_characters_is_valid(name in random_length_url_safe_string(1, 64)) {
            prop_assert!(Name::try_from(name).is_ok());
        }

        #[test]
        fn any_username_of_65_valid_characters_is_invalid(name in random_length_url_safe_string(65, 65)) {
            prop_assert!(Name::try_from(name).is_err());
        }
    }

    fn random_length_url_safe_string(min: usize, max: usize) -> impl Strategy<Value = String> {
        let max = rand::random_range(min..=max);

        url_safe_string(min, max)
    }

    fn url_safe_string(min: usize, max: usize) -> impl Strategy<Value = String> {
        string_regex(&format!("[A-Za-z0-9\\-_.~]{{{min},{max}}}"))
            .expect("invalid regex")
            .boxed()
    }
}
