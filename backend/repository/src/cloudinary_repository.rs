use cloudinary::upload::{result::UploadResult, Source, Upload, UploadOptions};
use dotenvy::dotenv;
use md5;
use models::domain::{user::User, Id};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct GenericRepository {}

impl GenericRepository {}

pub trait CloudinaryRepository {
    async fn upload_image(&self, image: String) -> Result<String, Box<dyn std::error::Error>>;
}

impl CloudinaryRepository for GenericRepository {
    async fn upload_image(&self, image: String) -> Result<String, Box<dyn std::error::Error>> {
        // Cloudinary Credentials Setup
        let api_key = dotenvy::var("CLOUDINARY_API_KEY").expect("env variables not set");
        let cloud_name = dotenvy::var("CLOUDINARY_CLOUD_NAME").expect("env variables not set");
        let api_secret = dotenvy::var("CLOUDINARY_API_SECRET").expect("env variables not set");

        let options = UploadOptions::new()
            .set_folder("iis_project".to_string())
            .set_public_id(format!("{:x}", md5::compute(&image)));

        let upload = Upload::new(api_key, cloud_name, api_secret);

        let result = upload.image(Source::DataUrl(image.into()), &options).await;
        // .expect("Error uploading image");

        match result {
            // If successful, return the secure URL
            Ok(result) => {
                if let UploadResult::Response(response) = result {
                    Ok(response.secure_url)
                } else {
                    Err("Failed to upload an image".to_string().into())
                }
            }
            // Handle errors
            Err(_) => Err("Failed to upload an image".to_string().into()),
        }
    }
}
