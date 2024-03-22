use anyhow::Error;
use std::collections::HashMap;
use strfmt::strfmt;

pub fn construct_full_image_path(
    post_guid_as_str: &str,
    user_guid_as_str: &str,
    uf_image_format: Option<&str>,
    default_location: &str,
) -> Result<String, async_graphql::Error> {
    Ok(format!(
        "{}/{}{}",
        default_location,
        construct_image_user_dir(&post_guid_as_str, &user_guid_as_str).unwrap(),
        construct_image_title(uf_image_format.unwrap_or("png")).unwrap()
    ))
}

pub fn map_image_type(image_type: &str) -> Result<String, async_graphql::Error> {
    let mapped_image_type: Result<String, async_graphql::Error> = match image_type {
        "image/png" => Ok("png".to_string()),
        "image/jpg" => Ok("jpg".to_string()),
        "image/jpeg" => Ok("jpeg".to_string()),
        _ => Err(async_graphql::Error::new("Image format is wrong!")),
    };
    mapped_image_type
}

pub fn construct_image_user_dir(
    post_guid_as_str: &str,
    user_guid_as_str: &str,
) -> Result<String, Error> {
    let uf_image_dir_user_format: String =
        dotenv::var("IMAGES_FORMAT").unwrap_or("{user}/{post}/".to_string());
    let mut vars: HashMap<String, &str> = HashMap::new();
    vars.insert("user".to_string(), user_guid_as_str);
    vars.insert("post".to_string(), &*post_guid_as_str);
    Ok(strfmt(&uf_image_dir_user_format, &vars)?)
}

pub fn construct_image_title(f_image_type: &str) -> Result<String, Error> {
    let uf_image_name: String =
        dotenv::var("IMAGES_NAME").unwrap_or("main_image.{content_type}".to_string());
    let mut vars: HashMap<String, &str> = HashMap::new();
    vars.insert("content_type".to_string(), &*f_image_type);
    Ok(strfmt(&uf_image_name, &vars)?)
}
