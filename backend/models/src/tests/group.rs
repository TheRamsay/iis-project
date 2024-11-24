#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use uuid::Uuid;

    use crate::domain::{email::Email, group::Group, user::User, Id};

    #[test]
    fn group() {
        let group = Group::new("ahoj".into(), Id::gen(), Id::gen());

        assert!(group.is_ok());

        let group = group.unwrap();
        assert!(group.id.id != Uuid::nil());
        assert!(group.name == "ahoj");
        assert!(group.admin_id.id != Uuid::nil());
    }

    #[test]
    fn group_name_too_short() {
        let group = Group::new("a".into(), Id::gen(), Id::gen());

        assert!(group.is_err());
    }

    #[test]
    fn group_name_too_long() {
        let group = Group::new(repeat("X").take(256).collect(), Id::gen(), Id::gen());

        assert!(group.is_err());
    }
}
