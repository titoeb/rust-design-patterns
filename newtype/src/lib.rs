use std::fmt;
pub struct SecretString(String);

impl SecretString {
    fn new(secret: String) -> Self {
        SecretString(secret)
    }
}

impl fmt::Display for SecretString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.chars().map(|_| '*').collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            format!("{}", SecretString::new("SECRET".to_string())),
            "******".to_string()
        )
    }
}
