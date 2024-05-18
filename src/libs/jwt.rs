//! // shared key
//! // HS256
//! let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
//! // public private key
//! // better cache EncodingKey in lazy_static
//! // RSA
//! let token = encode(&Header::new(Algorithm::RS256), &my_claims, &EncodingKey::from_rsa_pem(include_bytes!("privkey.pem"))?)?;
use super::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

lazy_static::lazy_static! {
    static ref SECRET: Vec<u8> = {
        let foo = dotenvy::var("JWT_SECRET").expect("reading JWT_SECRET env");
        foo.as_bytes().to_vec()
    };

    // let val = Validation::new(jsonwebtoken::Algorithm::default());
    static ref VALIDATION: Validation = Validation::default();
    static ref ENCODING_KEY: EncodingKey = EncodingKey::from_secret(&SECRET);
    static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(&SECRET);
}

pub fn one_week_expiration() -> usize {
    Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("valid timestamp")
        .timestamp() as usize
}

#[allow(unused)]
pub fn five_second_expiration() -> usize {
    Utc::now()
        .checked_add_signed(chrono::Duration::nanoseconds(1))
        .expect("valid timestamp")
        .timestamp() as usize
}

pub fn sign<T>(claims: T) -> Result<String> where T: Serialize {
    encode(&Header::default(), &claims, &ENCODING_KEY).map_err(Error::fatal)
}

pub fn verify<T>(token: String) -> Result<T> where T: DeserializeOwned {
    decode::<T>(&token, &DECODING_KEY, &VALIDATION).map(|e|e.claims).map_err(jwt_decode_err_map)
}

fn jwt_decode_err_map(value: jsonwebtoken::errors::Error) -> Error {
    use jsonwebtoken::errors::ErrorKind::*;
    match value.kind() {
        InvalidToken | InvalidSignature | InvalidEcdsaKey | InvalidRsaKey(_) |
        MissingRequiredClaim(_) | Base64(_) | Json(_)
            => Error::InvalidToken(value.to_string()),

        InvalidIssuer | MissingAlgorithm | InvalidAlgorithm | InvalidAudience |
        InvalidSubject | ImmatureSignature
            => Error::InvalidToken(value.to_string()),

        ExpiredSignature => Error::InvalidToken("Token expired, please issue a new token".to_owned()),

        RsaFailedSigning | InvalidAlgorithmName | InvalidKeyFormat | Utf8(_) | Crypto(_) | _
            => Error::fatal(value),
    }
}

