use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};

#[derive(Debug, Clone)]
pub struct Password(Secret<String>);

impl PartialEq for Password { 
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret() 
    }
}

impl Password {
    pub fn parse(input: Secret<String>) -> Result<Password> {
        if !validate_password(&input) {
            return Err(eyre!("Failed to parse string to a Password type"));
        } 
        Ok(Self(input))
    }
}

impl AsRef<Secret<String>> for Password { 
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

fn validate_password(s: &Secret<String>) -> bool {
    s.expose_secret().len() >= 8
}

#[cfg(test)]
mod tests {
    use super::Password;

    use fake::faker::internet::en::Password as FakePassword;
    use fake::Fake;
    use secrecy::Secret;

    #[test]
    fn should_return_password_err_when_not_properly_parsed() {
        let results = [
            Password::parse(Secret::new("".to_string())),
            Password::parse(Secret::new("1".to_string())),
            Password::parse(Secret::new("12".to_string())),
            Password::parse(Secret::new("123".to_string())),
            Password::parse(Secret::new("1234".to_string())),
            Password::parse(Secret::new("12345".to_string())),
            Password::parse(Secret::new("123456".to_string())),
            Password::parse(Secret::new("1234567".to_string())),
        ];

        assert!(results.iter().all(|r| r.is_err()))
    }

    #[test]
    fn should_return_password_ok_when_properly_parsed() {
        let results = [
            Password::parse(Secret::new("password".to_string())),
            Password::parse(Secret::new("some long passphrase that should also work".to_string())),
        ];

        assert!(results.iter().all(|r| r.is_ok()))
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub Secret<String>); 

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..30).fake_with_rng(g);
            Self(Secret::new(password)) 
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(valid_password.0).is_ok()
    }
}