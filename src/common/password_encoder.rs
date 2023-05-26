use bcrypt::{hash, verify, DEFAULT_COST};

pub fn encode_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_password() {
        let password = "password";
        let encoded_password = encode_password(password);

        assert_ne!(password, encoded_password);
    }

    #[test]
    fn test_verify_password() {
        let password = "password";
        let encoded_password = encode_password(password);

        assert!(verify_password(password, &encoded_password));
    }
}
