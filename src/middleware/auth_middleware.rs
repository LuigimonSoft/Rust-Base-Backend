use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use warp::{Filter, Rejection};
use crate::models::auth_model::Claims;
use std::convert::Infallible;

const JWT_SECRET: &[u8] = b"your-secret-key"; // In production, use environment variable

pub fn with_auth() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
    warp::header::optional::<String>("authorization")
        .and_then(|auth_header: Option<String>| async move {
            match auth_header {
                Some(header) => {
                    if let Some(token) = header.strip_prefix("Bearer ") {
                        match decode_jwt(token) {
                            Ok(claims) => Ok(claims),
                            Err(_) => Err(warp::reject::custom(AuthError::InvalidToken)),
                        }
                    } else {
                        Err(warp::reject::custom(AuthError::MissingToken))
                    }
                }
                None => Err(warp::reject::custom(AuthError::MissingToken)),
            }
        })
}

fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(JWT_SECRET);
    let validation = Validation::new(Algorithm::HS256);
    
    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
}

pub fn create_jwt(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    use chrono::{Utc, Duration};

    let now = Utc::now();
    let expires_in = Duration::hours(24);
    let exp = (now + expires_in).timestamp();
    
    let claims = Claims {
        sub: username.to_owned(),
        exp,
        iat: now.timestamp(),
    };

    let encoding_key = EncodingKey::from_secret(JWT_SECRET);
    encode(&Header::default(), &claims, &encoding_key)
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    MissingToken,
}

impl warp::reject::Reject for AuthError {}