use crate::error::FileServerError;
use std::env;

/// Function to check API key in Authorization header.
/// If the API key is not valid, it will return an error.
///
/// I implemented this function because it has some issues with Swagger UI when using middleware or guard.
///
/// # Arguments
///
/// * `req` - Request
pub(crate) fn check_api_key(req: &actix_web::HttpRequest) -> Result<(), FileServerError> {
    if req.head().headers().get("Authorization").is_none() {
        return Err(FileServerError::Unauthorized {
            message: "Authorization header not found".to_string(),
        });
    }

    let token = req
        .head()
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    // println!("token: {}, api_key: {}", token, hex::encode(&api_key));

    if token != format!("Bearer {}", hex::encode(api_key)) {
        return Err(FileServerError::Unauthorized {
            message: "Invalid API key".to_string(),
        });
    }

    Ok(())
}
