#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use uuid::Uuid;

    use crate::domain::{email::Email, group::Group, post_comment::PostComment, user::User, Id};

    #[test]
    fn post_comment() {
        let post_comment = PostComment::new(Id::gen(), Id::gen(), "ahoj".into(), None);

        assert!(post_comment.is_ok());

        let post_comment = post_comment.unwrap();

        assert!(post_comment.id.id != Uuid::nil());
        assert!(post_comment.post_id.id != Uuid::nil());
        assert!(post_comment.user_id.id != Uuid::nil());
        assert!(post_comment.content == "ahoj");
        assert!(post_comment.parent_id.is_none());
    }

    #[test]
    fn post_comment_content_too_short() {
        let post_comment = PostComment::new(Id::gen(), Id::gen(), "".into(), None);

        assert!(post_comment.is_err());
    }

    #[test]
    fn post_comment_content_too_long() {
        let post_comment =
            PostComment::new(Id::gen(), Id::gen(), repeat("X").take(256).collect(), None);

        assert!(post_comment.is_err());
    }
}
