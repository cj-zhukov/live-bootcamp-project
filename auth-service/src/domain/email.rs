#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    #[test]
    fn test_valid_parse_email() {
        let input = "email@com";
        let res = Email::parse(input).unwrap();
        assert_eq!(res.0, input);
    }

    #[test]
    fn test_invalid_parse_email() {
        let input = "emailcom";
        let res = Email::parse(input);
        assert_eq!(res, Err(format!("failed parsing email: {}", input)));
    }
}