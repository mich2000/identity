use chrono::prelude::*;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use identity_dal::user::identity_user::IdentityUser;
use identity_dal::traits::t_user_manager::UserStoreTrait;
use crate::store::Store;
use crate::traits::token::TokenContainerTrait;
use crate::IdentityError;
use crate::util::{ self, get_value_from_key };

lazy_static! {
    static ref ISSUER: String = get_value_from_key("PERSON_ISSUER")
    .expect("PERSON_ISSUER variable not found in the .env config file or as environment variable");
    static ref SECRET: String = get_value_from_key("PERSON_SECRET")
    .expect("PERSON_SECRET variable not found in the .env config file or as environment variable");
    static ref EXPIRATION : i64 = get_value_from_key("PERSON_EXPIRATION")
    .expect("PERSON_EXPIRATION variable not found in the .env config file or as environment variable")
    .parse::<i64>().expect("Could not parse this string to i64");
    static ref EXPIRATION_CHANGE_PWD : i64 = get_value_from_key("PERSON_EXPIRATION_CHANGE_PWD")
    .expect("PERSON_EXPIRATION_CHANGE_PWD variable not found in the .env config file or as environment variable")
    .parse::<i64>().expect("Could not parse this string to i64");
}

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
    #[serde(with = "util::jwt_numeric_date")]
    pub exp: DateTime<Utc>,
    #[serde(with = "util::jwt_numeric_date")]
    pub iat: DateTime<Utc>
}

impl Claim {
    /**
     * From a sub(id of an user) a claim is made. The expiration time comes from the .env file on the line person_expiration. The issue comes form the .env file on the line person_issuer. The iat property is just the datetime of today and the exp is the expiration plus the datetime of today.
     */
    pub fn new_read_write_claim(subject: &str) -> Result<Claim, IdentityError> {
        if subject.is_empty() {
            warn!("The subject of the jwt claim is empty");
            return Err(IdentityError::SubjectOfTokenIsEmpty)
        }
        let today = Utc::now();
        Ok(Claim {
            sub: subject.to_string(),
            iss: ISSUER.clone(),
            exp: today + chrono::Duration::seconds(*EXPIRATION),
            iat: today,
        })
    }

    pub fn new_change_password_claim(subject: &str) -> Result<Claim, IdentityError> {
        if subject.is_empty() {
            warn!("The subject of the jwt claim is empty");
            return Err(IdentityError::SubjectOfTokenIsEmpty)
        }
        let today = Utc::now();
        Ok(Claim {
            sub: subject.to_string(),
            iss: ISSUER.clone(),
            exp: today + chrono::Duration::seconds(*EXPIRATION_CHANGE_PWD),
            iat: today
        })
    }

    /**
     * Returns a string token from the claim. The encoding secret comes from the .env file from the line person_secret. An error is thrown when the token creation fails.
     */
    pub fn token_from_user(&self) -> Result<String, IdentityError> {
        match encode(&Header::default(), &self, SECRET.as_ref()) {
            Ok(token) => {
                info!("A token has been made from a claim");
                Ok(token)
            }
            Err(e) => {
                warn!("A token couldn't be made out of a claim. Reason: {}", e);
                Err(IdentityError::TokenCannotBeMadeFromClaim)
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
    pub fn decode_token(token: &str) -> Result<TokenData<Claim>, IdentityError> {
        if token.is_empty() {
            warn!("A token string cannot be empty");
            return Err(IdentityError::TokenIsEmpty)
        }
        let mut validate: Validation = Validation::default();
        validate.iss = Some(ISSUER.clone());
        match decode::<Claim>(
            &token,
            SECRET.as_ref(),
            &validate,
        ) {
            Ok(c) => Ok(c),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => {
                    warn!("jwt token is invalid");
                    Err(IdentityError::TokenIsInvalid)
                }
                ErrorKind::InvalidIssuer => {
                    warn!("jwt token issuer is invalid");
                    Err(IdentityError::IssuerIsInvalid)
                }
                ErrorKind::ExpiredSignature => {
                    warn!("Signature of jwt token has been expired");
                    Err(IdentityError::SignatureHasExpired)
                }
                _ => {
                    warn!("Some other errors with a jwt token decoding");
                    Err(IdentityError::CustomError("Some other errors".to_string()))
                }
            },
        }
    }

    /**
     * Function used to take in viewmodels that posses the trait TokenContainerTrait so that it can decode from a token string in a viewmodel and return a result where in there is TokenData<Claim>.
     */
    pub fn decode_token_viewmodel<T : TokenContainerTrait + Send + 'static>(token: &T) -> Result<TokenData<Claim>, IdentityError> {
        Claim::decode_token(token.get_token())
    }

    /**
     * Token function that decodes a token and makes a claim out of it. From the claim it takes the subject which is the user id and it seeks based on this the user associated with that id. If the user isn't found an error is then returned.
     */
    pub fn token_to_user(token: &str, db: &Store) -> Result<IdentityUser, IdentityError> {
        match Claim::decode_token(token) {
            Ok(token) => match db.get_user_by_uuid(&token.claims.sub) {
                Some(user) => Ok(user),
                None => {
                    warn!("The subject of the token is not mapped to an user.");
                    Err(IdentityError::UserNotFound)
                }
            },
            Err(e) => {
                warn!("Token is invalid: {}", e);
                Err(e)
            }
        }
    }
}