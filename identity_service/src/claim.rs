use chrono::prelude::*;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use identity_dal::user::identity_user::IdentityUser;
use identity_dal::traits::t_user_manager::UserStoreTrait;
use crate::store::Store;
use crate::traits::token::TokenContainerTrait;

lazy_static! {
    static ref ENCODER: EncodingKey = EncodingKey::from_secret(SECRET.as_ref());
    static ref ISSUER: String = dotenv::var("person_issuer").expect("Could not parse the person_issuer line in the .env config file.");
    static ref SECRET: String = dotenv::var("person_secret").expect("Could not parse the person_secret line in the .env config file.");
}

static EXPIRATION : i64 = 22;

/**
 * Claim is used to prove authorization for an user for a certain amount of time.
 *
 * Attributes:
 * * sub : is an id of an user
 * * iss : is the issuer of the claim
 * * exp : datetime which indicates the date that it will be valid
 * * iat : datetime the claim was issued
 * * is_admin : Claim that is used to identify if the user is an administrator
 */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    pub sub: String,
    pub iss: String,
    #[serde(with = "jwt_numeric_date")]
    pub exp: DateTime<Utc>,
    #[serde(with = "jwt_numeric_date")]
    pub iat: DateTime<Utc>,
}

impl Claim {
    /**
     * From a sub(id of an user) a claim is made. The expiration time comes from the .env file on the line person_expiration. The issue comes form the .env file on the line person_issuer. The iat property is just the datetime of today and the exp is the expiration plus the datetime of today.
     */
    pub fn new_claim(subject: &str) -> Result<Claim, &'static str> {
        if subject.is_empty() {
            warn!("The subject of the jwt claim is empty");
            return Err("The subject can't be empty");
        }
        let today = Utc::now();
        Ok(Claim {
            sub: subject.to_string(),
            iss: ISSUER.clone(),
            exp: today + chrono::Duration::seconds(EXPIRATION),
            iat: today,
        })
    }

    /**
     * Returns a string token from the claim. The encoding secret comes from the .env file from the line person_secret. An error is thrown when the token creation fails.
     */
    pub fn token_from_user(&self) -> Result<String, &'static str> {
        match encode(&Header::default(), &self, &ENCODER) {
            Ok(token) => {
                info!("A token has been made from a claim");
                Ok(token)
            }
            Err(e) => {
                warn!("A token couldn't be made out of a claim. Reason: {}", e);
                Err("Couldn't create a token out of a claim")
            }
        }
    }

    /**
     * Function that decodes a token string returning a claim.
     *
     * An error can be thrown when:
     * * a token is empty
     * * Whenever the issuer of the decoded token is not equal to the issuer in the .env file
     * * token is invalid
     */
    pub fn decode_token(token: &str) -> Result<TokenData<Claim>, &'static str> {
        if token.is_empty() {
            warn!("A token string cannot be empty");
            return Err("A token can't be empty");
        }
        let mut validate: Validation = Validation::default();
        validate.iss = Some(ISSUER.clone());
        match decode::<Claim>(
            &token,
            &DecodingKey::from_secret(&SECRET.as_bytes()),
            &validate,
        ) {
            Ok(c) => Ok(c),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => {
                    warn!("jwt token is invalid");
                    Err("Token is invalid")
                }
                ErrorKind::InvalidIssuer => {
                    warn!("jwt token issuer is invalid");
                    Err("Issuer is invalid")
                }
                ErrorKind::ExpiredSignature => {
                    warn!("Signature of jwt token has been expired");
                    Err("Signature has been expired")
                }
                _ => {
                    warn!("Some other errors with a jwt token decoding");
                    Err("Some other errors")
                }
            },
        }
    }

    /**
     * Function used to take in viewmodels that posses the trait TokenContainerTrait so that it can decode from a token string in a viewmodel and return a result where in there is TokenData<Claim>.
     */
    pub fn decode_token_viewmodel<T : TokenContainerTrait + Send + 'static>(token: &T) -> Result<TokenData<Claim>, &'static str> {
        Claim::decode_token(token.get_token())
    }

    /**
     * Token function that decodes a token and makes a claim out of it. From the claim it takes the subject which is the user id and it seeks based on this the user associated with that id. If the user isn't found an error is then returned.
     */
    pub fn token_to_user(token: &str, db: &Store) -> Result<IdentityUser, &'static str> {
        match Claim::decode_token(token) {
            Ok(token) => match db.get_user_by_uuid(&token.claims.sub) {
                Some(user) => Ok(user),
                None => {
                    warn!("The subject of the token is not mapped to an user.");
                    Err("Could not find the user")
                }
            },
            Err(e) => {
                warn!("Token is invalid: {}", e);
                Err(e)
            }
        }
    }
}

/**
 * Link for the converter: https://github.com/Keats/jsonwebtoken/blob/master/examples/custom_chrono.rs
 */
mod jwt_numeric_date {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.timestamp_opt(i64::deserialize(deserializer)?, 0)
            .single()
            .ok_or_else(|| serde::de::Error::custom("invalid Unix timestamp value"))
    }
}
