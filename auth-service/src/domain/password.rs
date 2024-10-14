#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(input: &str) -> Result<Self, String> {
        if input.len() < 8 {
            return Err(format!("failed parsing passsword: {}", input));
        } 
        Ok(Self(input.to_string()))
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
