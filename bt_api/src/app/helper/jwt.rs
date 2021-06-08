use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;

pub fn jwt_encode<T: Serialize + DeserializeOwned>(claims: &T) -> String {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )
    .unwrap()
}

fn get_secret() -> String {
    env::var("JWT_SECRET").expect("You need a JWT_SECRET env var")
}

pub fn jwt_decode<T: Serialize + DeserializeOwned>(token: &str) -> Result<T, Error> {
    decode::<T>(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
}
