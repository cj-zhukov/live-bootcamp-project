use color_eyre::eyre::{eyre, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(input: &str) -> Result<Self> {
        if input.len() < 8 {
            // return Err(format!("failed parsing passsword: {}", input));
            return Err(eyre!("Failed parsing password"));
        } 
        Ok(Self(input.to_string()))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    // use fake::faker::internet::en::Password as FakePassword;
    // use fake::Fake;

    #[test]
    fn should_return_password_err_when_not_properly_parsed() {
        let results = [
            Password::parse(""),
            Password::parse("1"),
            Password::parse("12"),
            Password::parse("123"),
            Password::parse("1234"),
            Password::parse("12345"),
            Password::parse("123456"),
            Password::parse("1234567"),
        ];

        assert!(results.iter().all(|r| r.is_err()))
    }

    #[test]
    fn should_return_password_ok_when_properly_parsed() {
        let results = [
            Password::parse("password"),
            Password::parse("some long passphrase that should also work"),
        ];

        assert!(results.iter().all(|r| r.is_ok()))
    }

    // TODO panics
    // #[derive(Debug, Clone)]
    // struct ValidPasswordFixture(pub String);

    // impl quickcheck::Arbitrary for ValidPasswordFixture {
    //     fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
    //         let password = FakePassword(8..30).fake_with_rng(g);
    //         Self(password)
    //     }
    // }
    // #[quickcheck_macros::quickcheck]
    // fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
    //     Password::parse(&valid_password.0).is_ok()
    // }
}