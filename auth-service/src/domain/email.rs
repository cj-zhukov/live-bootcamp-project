use std::hash::Hash;

use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Email(Secret<String>);

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl Eq for Email {}

fn validate_email(input: &Secret<String>) -> bool {
    input.expose_secret().contains('@')
}

impl Email {
    pub fn parse(input: Secret<String>) -> Result<Self> {
        if !validate_email(&input) {
            return Err(eyre!("Failed parsing email"));
        } 
        Ok(Self(input))
    }
}

impl AsRef<Secret<String>> for Email {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // use fake::faker::internet::en::SafeEmail;
    // use fake::Fake;
    use secrecy::Secret; 

    #[test]
    fn should_return_email_ok_when_properly_parsed() {
        let results = [
            Email::parse(Secret::new("some@value.com".to_string())),
            Email::parse(Secret::new("some.other@value.com".to_string())),
            Email::parse(Secret::new("some-value123@value.com".to_string())),
        ];

        assert!(results.iter().all(|r| r.is_ok()))
    }

    #[test]
    fn should_return_email_err_when_not_properly_parsed() {
        let results = [
            Email::parse(Secret::new("some]value.com".to_string())), 
            // Email::parse(Secret::new("".to_string())),
        ];

        assert!(results.iter().all(|r| r.is_err()))
    }

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
    //     Email::parse(Secret::new(valid_email.0)).is_ok() 
    // }
}