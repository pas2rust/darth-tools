use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ChronoTrait;

use super::darth_tools::DarthTools;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Token {
    pub items: Value,
    pub exp: usize,
}

pub trait JsonwebtokenTrait {
    fn new_jsonwebtoken_hs256_decode(token: String, secret: &str) -> Result<Token, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs384_decode(token: String, secret: &str) -> Result<Token, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs512_decode(token: String, secret: &str) -> Result<Token, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs256_encode(
        items: Value,
        exp: usize,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs384_encode(
        items: Value,
        exp: usize,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error>;
    fn new_jsonwebtoken_hs512_encode(
        items: Value,
        exp: usize,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error>;
}

impl JsonwebtokenTrait for DarthTools {
    fn new_jsonwebtoken_hs256_decode(token: String, secret: &str) -> Result<Token, jsonwebtoken::errors::Error> {
        let res = decode::<Token>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )?;
        Ok(res.claims)
    }
    fn new_jsonwebtoken_hs384_decode(token: String, secret: &str) -> Result<Token, jsonwebtoken::errors::Error> {
        let res = decode::<Token>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS384),
        )?;
        Ok(res.claims)
    }
    fn new_jsonwebtoken_hs512_decode(token: String, secret: &str) -> Result<Token, jsonwebtoken::errors::Error> {
        let res = decode::<Token>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS512),
        )?;
        Ok(res.claims)
    }
    fn new_jsonwebtoken_hs256_encode(
        items: Value,
        exp: usize,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = DarthTools::new_date_utc_now().timestamp() as usize;
        let exp = now + exp;
        let my_claims = Token { items, exp };
        let header = Header::new(jsonwebtoken::Algorithm::HS256);
        encode(&header, &my_claims, &EncodingKey::from_secret(secret.as_ref()))
    }
    fn new_jsonwebtoken_hs384_encode(
        items: Value,
        exp: usize,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = DarthTools::new_date_utc_now().timestamp() as usize;
        let exp = now + exp;
        let my_claims = Token { items, exp };
        let header = Header::new(jsonwebtoken::Algorithm::HS384);
        encode(&header, &my_claims, &EncodingKey::from_secret(secret.as_ref()))
    }
    fn new_jsonwebtoken_hs512_encode(
        items: Value,
        exp: usize,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = DarthTools::new_date_utc_now().timestamp() as usize;
        let exp = now + exp;
        let my_claims = Token { items, exp };
        let header = Header::new(jsonwebtoken::Algorithm::HS512);
        encode(&header, &my_claims, &EncodingKey::from_secret(secret.as_ref()))
    }
}
