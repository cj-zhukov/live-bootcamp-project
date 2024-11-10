use color_eyre::eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(input: &str) -> Result<Self> {
        if !input.contains('@') || input.is_empty() {
            // return Err(format!("failed parsing email: {}", input));
            return Err(eyre!("Failed parsing email"));
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

    // use fake::faker::internet::en::SafeEmail;
    // use fake::Fake;

    #[test]
    fn should_return_email_ok_when_properly_parsed() {
        let results = [
            Email::parse("some@value.com"),
            Email::parse("some.other@value.com"),
            Email::parse("some-value123@value.com"),
        ];

        assert!(results.iter().all(|r| r.is_ok()))
    }

    #[test]
    fn should_return_email_err_when_not_properly_parsed() {
        let results = [
            Email::parse("some]value.com"), 
            Email::parse(""),
        ];

        assert!(results.iter().all(|r| r.is_err()))
    }

    // TODO panics 
    // #[derive(Debug, Clone)]
    // struct ValidEmailFixture(pub String);

    // impl quickcheck::Arbitrary for ValidEmailFixture {
    //     fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
    //         let email = SafeEmail().fake_with_rng(g);
    //         Self(email)
    //     }
    // }

    // #[quickcheck_macros::quickcheck]
    // fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
    //     Email::parse(&valid_email.0).is_ok()
    // }
}