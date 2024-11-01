use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(input: &str) -> Result<Self, String> {
        if !input.contains('@') || input.is_empty() {
            return Err(format!("failed parsing email: {}", input));
        } 
        Ok(Self(input.to_string()))
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn empty_string_is_rejected() {
        let email = "";
        let res = Email::parse(email);
        assert!(res.is_err());
    }

    #[test]
    fn test_valid_parse_email() {
        let email = "email@com";
        let res = Email::parse(email);
        assert!(res.is_ok());
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "emailcom";
        let res = Email::parse(email);
        assert!(res.is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(&valid_email.0).is_ok()
    }
}