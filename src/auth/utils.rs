use jsonwebtoken::errors::Error;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::auth::models::Claims;
use crate::user::models::User;

pub fn encode_token(
    user: &User,
    exp: usize,
    token_type: String,
    secret: &str,
) -> Result<String, Error> {
    let claims = Claims::from_user(user, token_type, exp);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}
