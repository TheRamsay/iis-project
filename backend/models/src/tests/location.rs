#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::domain::{email::Email, group::Group, location::Location, user::User, Id};

    #[test]
    fn location() {
        let location = Location::new(
            Some("https://www.krejzac.cz".into()),
            "ahoj".into(),
            0.0,
            0.0,
        );

        assert!(location.is_ok());

        let location = location.unwrap();
        assert!(location.id.id != Uuid::nil());
        assert!(location.name == "ahoj");
        assert!(location.picture_url.unwrap() == "https://www.krejzac.cz");
        assert!(location.latitude == 0.0);
        assert!(location.longitude == 0.0);
    }

    #[test]
    fn location_invalid_picture_url() {
        let location = Location::new(Some("krejzac".into()), "ahoj".into(), 0.0, 0.0);

        assert!(location.is_err());
    }
}
