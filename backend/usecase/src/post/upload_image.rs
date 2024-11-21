use models::{
    domain::{
        post::{Post, PostType, PostVisibilityType},
        post_visibility::PostVisibility,
        Id,
    },
    errors::AppResult,
};
use repository::{cloudinary_repository::CloudinaryRepository, post_repository::PostRepository};
use uuid::Uuid;

#[derive(Debug)]
pub struct UploadImageInput {
    pub image: String,
}

pub struct UploadImageOutput {
    pub url: String,
}

pub struct UploadImageUseCase<T>
where
    T: CloudinaryRepository,
{
    cloudinary_repository: T,
}

impl<T> UploadImageUseCase<T>
where
    T: CloudinaryRepository,
{
    pub fn new(cloudinary_repository: T) -> Self {
        Self {
            cloudinary_repository,
        }
    }

    pub async fn execute(&self, input: UploadImageInput) -> AppResult<UploadImageOutput> {
        let result = self.cloudinary_repository.upload_image(input.image).await;

        match result {
            Ok(url) => Ok(UploadImageOutput { url }),
            Err(_) => Err(anyhow::anyhow!("Failed to upload an image").into()),
        }
    }
}
