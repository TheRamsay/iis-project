#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use uuid::Uuid;

    use crate::domain::{email::Email, user::User, Id};

    #[test]
    fn user() {
        let user = User::new(
            "ramsay".into(),
            "dominik_huml".into(),
            Email::new("dh@vutbr.cz".into()).unwrap(),
            Some("https://www.krejzac.cz".into()),
            crate::domain::user::UserType::Regular,
            Id::new(Uuid::new_v4()),
            "password_hash".into(),
        );

        assert!(user.is_ok());

        let user = user.unwrap();
        assert!(user.id.id != Uuid::nil());
        assert!(user.display_name == "ramsay");
        assert!(user.username == "dominik_huml");
        assert!(user.email.value == "dh@vutbr.cz");
        assert!(user.avatar_url.unwrap() == "https://www.krejzac.cz");
        assert!(user.user_type == crate::domain::user::UserType::Regular);
        assert!(user.wall_id.id != Uuid::nil());
        assert!(user.is_blocked == false);
        assert!(user.password_hash == "password_hash");
    }

    #[test]
    fn user_display_name_too_short() {
        let user = User::new(
            "d".into(),
            "efweed".into(),
            Email::new("dh@vutbr.cz".into()).unwrap(),
            None,
            crate::domain::user::UserType::Regular,
            Id::new(Uuid::new_v4()),
            "password_hash".into(),
        );

        assert!(user.is_err());
    }

    #[test]
    fn user_username_too_short() {
        let user = User::new(
            "ramsay".into(),
            "d".into(),
            Email::new("dh@vutbr.cz".into()).unwrap(),
            None,
            crate::domain::user::UserType::Regular,
            Id::new(Uuid::new_v4()),
            "password_hash".into(),
        );

        assert!(user.is_err());
    }

    #[test]
    fn user_display_name_too_long() {
        let user = User::new(
            repeat("X").take(256).collect(),
            "dominik_huml".into(),
            Email::new("dh@vutbr.cz".into()).unwrap(),
            None,
            crate::domain::user::UserType::Regular,
            Id::new(Uuid::new_v4()),
            "password_hash".into(),
        );

        assert!(user.is_err());
    }

    #[test]
    fn user_username_too_long() {
        let user = User::new(
            "dominik_huml".into(),
            repeat("X").take(256).collect(),
            Email::new("dh@vutbr.cz".into()).unwrap(),
            None,
            crate::domain::user::UserType::Regular,
            Id::new(Uuid::new_v4()),
            "password_hash".into(),
        );

        assert!(user.is_err());
    }

    #[test]
    fn user_invalid_avatar_url() {
        let user = User::new(
            "dominik_huml".into(),
            "dominik_huml".into(),
            Email::new("dh@vutbr.cz".into()).unwrap(),
            Some("invalid_url".into()),
            crate::domain::user::UserType::Regular,
            Id::new(Uuid::new_v4()),
            "password_hash".into(),
        );

        assert!(user.is_err());
    }
}
