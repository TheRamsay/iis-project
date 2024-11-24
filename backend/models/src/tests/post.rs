#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use uuid::Uuid;

    use crate::domain::{
        email::Email, group::Group, post::Post, post_comment::PostComment, user::User, Id,
    };

    #[test]
    fn post() {
        let post = Post::new(
            "Muj post".into(),
            "hmmmmmmm".into(),
            Id::gen(),
            crate::domain::post::PostType::Photo,
            "https://www.krejzac.cz".into(),
            crate::domain::post::PostVisibilityType::Private,
            None,
        );

        assert!(post.is_ok());

        let post = post.unwrap();

        assert!(post.id.id != Uuid::nil());
        assert!(post.title == "Muj post");
        assert!(post.description == "hmmmmmmm");
        assert!(post.author_id.id != Uuid::nil());
        assert!(post.post_type == crate::domain::post::PostType::Photo);
        assert!(post.content_url == "https://www.krejzac.cz");
        assert!(post.visibility == crate::domain::post::PostVisibilityType::Private);
        assert!(post.location_id.is_none());
    }

    #[test]
    fn post_title_too_short() {
        let post = Post::new(
            "aa".into(),
            "hmmmmmmm".into(),
            Id::gen(),
            crate::domain::post::PostType::Photo,
            "https://www.krejzac.cz".into(),
            crate::domain::post::PostVisibilityType::Private,
            None,
        );

        assert!(post.is_err());
    }

    #[test]
    fn post_title_too_long() {
        let post = Post::new(
            repeat("X").take(51).collect(),
            "hmmmmmmm".into(),
            Id::gen(),
            crate::domain::post::PostType::Photo,
            "https://www.krejzac.cz".into(),
            crate::domain::post::PostVisibilityType::Private,
            None,
        );

        assert!(post.is_err());
    }

    #[test]
    fn post_description_too_short() {
        let post = Post::new(
            "Muj post".into(),
            "aa".into(),
            Id::gen(),
            crate::domain::post::PostType::Photo,
            "https://www.krejzac.cz".into(),
            crate::domain::post::PostVisibilityType::Private,
            None,
        );

        assert!(post.is_err());
    }

    #[test]
    fn post_description_too_long() {
        let post = Post::new(
            "Muj post".into(),
            repeat("X").take(501).collect(),
            Id::gen(),
            crate::domain::post::PostType::Photo,
            "https://www.krejzac.cz".into(),
            crate::domain::post::PostVisibilityType::Private,
            None,
        );

        assert!(post.is_err());
    }

    #[test]
    fn post_content_url_invalid() {
        let post = Post::new(
            "Muj post".into(),
            "hmmmmmmm".into(),
            Id::gen(),
            crate::domain::post::PostType::Photo,
            "krejza".into(),
            crate::domain::post::PostVisibilityType::Private,
            None,
        );

        assert!(post.is_err());
    }
}
