use jsonwebtoken::{encode, errors::Error, DecodingKey, EncodingKey, Header};

use crate::{auth::models::Claims, user::models::User};

pub fn encode_token(
    user: &User,
    exp: usize,
    token_type: &str,
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

pub fn validate_token(token: &str, secret: &str) -> Result<Claims, Error> {
    let token = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;

    Ok(token.claims)
}
