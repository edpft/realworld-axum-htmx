use std::sync::LazyLock;

use regex::Regex;

pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = String;

    fn try_from(maybe_email: String) -> Result<Self, Self::Error> {
        if is_html5_email(&maybe_email) {
            Ok(Self(maybe_email))
        } else {
            Err(format!("'{maybe_email}' is not a valid email"))
        }
    }
}

fn is_html5_email(maybe_email: &str) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[\w.!#$%&'*+/=?^`{|}~-]+@[a-z\d](?:[a-z\d-]{0,61}[a-z\d])?(?:\.[a-z\d](?:[a-z\d-]{0,61}[a-z\d])?)*$").unwrap());
    RE.is_match(maybe_email)
}


#[cfg(test)]
mod tests {
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use proptest::prelude::*;

    use super::Email;

    #[test]
    fn a_valid_email_is_valid() {
        let email = String::from("user@example.com");

        assert!(Email::try_from(email).is_ok())
    }

    #[test]
    fn an_invalid_email_is_invalid() {
        let email = String::from("foo");

        assert!(Email::try_from(email).is_err())
    }

    proptest! {
        #[test]
        fn any_valid_email_is_valid(email in fake_email_strategy()) {
            prop_assert!(Email::try_from(email).is_ok());
        }
    }

    fn fake_email_strategy() -> impl Strategy<Value = String> {
        any::<u64>().prop_map(|_| SafeEmail().fake::<String>())
    }
}
