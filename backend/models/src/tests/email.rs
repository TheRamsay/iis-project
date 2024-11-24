#[cfg(test)]
mod tests {
    use crate::domain::email::Email;

    #[test]
    fn email_valid() {
        let email_str = "dominik@vutbr.cz";
        let email = Email::new(email_str.to_string());

        assert!(email.is_ok());
    }

    #[test]
    fn email_invalid() {
        let email_str = "dominikvutbr";
        let email = Email::new(email_str.to_string());

        assert!(email.is_err());
    }
}
