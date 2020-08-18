use std::{error::Error, fmt};

#[derive(Debug,serde::Deserialize,serde::Serialize)]
pub enum IdentityError {
    EmailNotCorrectFormat,
    EmailIsEmpty,
    EmailIsAlreadyTaken,
    IdIsAlreadyTaken,
    EmailAndPasswordIsEmpty,
    PasswordIsNotCorrect,
    PasswordIsEmpty,
    PasswordCannotBeMade,
    PasswordAndPasswordConfirmedNotEqual,
    FirstAndLastNameIsEmpty,
    FirstNameIsEmpty,
    LastNameIsEmpty,
    UserNotFound,
    UserCannotBeAdded,
    UserAlreadyPresent,
    UserIsNotPresent,
    UserDeleteFailed,
    UserCannotBeUpdated,
    IdEqualsAdmin,
    IdNotEqualToAdmin,
    AdminNotPresent,
    SubjectOfTokenIsEmpty,
    TokenCannotBeMadeFromClaim,
    TokenIsEmpty,
    TokenIsInvalid,
    IssuerIsInvalid,
    SignatureHasExpired,
    SmtpDomainNotGood,
    CouldNotSendEmail,
    CustomError(String)
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdentityError::EmailNotCorrectFormat => write!(f,"Email is not in the correct form"),
            IdentityError::EmailIsEmpty => write!(f,"Email cannot be empty"),
            IdentityError::EmailIsAlreadyTaken => write!(f,"User email is already taken"),
            IdentityError::IdIsAlreadyTaken => write!(f,"User id is already taken"),
            IdentityError::EmailAndPasswordIsEmpty => write!(f,"Email and password can't be equal to nothing"),
            IdentityError::PasswordIsNotCorrect => write!(f,"Password is not right"),
            IdentityError::PasswordIsEmpty => write!(f,"Password cannot be empty"),
            IdentityError::PasswordCannotBeMade => write!(f,"Password couldn't be made"),
            IdentityError::PasswordAndPasswordConfirmedNotEqual => write!(f,"Password and confirmed password aren't the same"),
            IdentityError::FirstAndLastNameIsEmpty => write!(f,"The first and last name can't be empty"),
            IdentityError::FirstNameIsEmpty => write!(f,"The firstname can't be empty"),
            IdentityError::LastNameIsEmpty => write!(f,"The last name can't be empty"),
            IdentityError::UserNotFound => write!(f,"User cannot be found"),
            IdentityError::UserCannotBeAdded => write!(f,"User cannot be added"),
            IdentityError::UserAlreadyPresent => write!(f,"User is already present"),
            IdentityError::UserIsNotPresent => write!(f,"User is not present"),
            IdentityError::UserDeleteFailed => write!(f,"The user's password wasn't correct or delete confirmation was not set to true"),
            IdentityError::UserCannotBeUpdated => write!(f,"User cannot be updated."),
            IdentityError::IdEqualsAdmin => write!(f,"The given id equals to the id of the admin"),
            IdentityError::IdNotEqualToAdmin => write!(f,"The given id isn't equals to the id of the admin"),
            IdentityError::AdminNotPresent => write!(f,"Admin is not present"),
            IdentityError::SubjectOfTokenIsEmpty => write!(f,"The subject is empty"),
            IdentityError::TokenCannotBeMadeFromClaim => write!(f,"Couldn't create a token out of a claim"),
            IdentityError::TokenIsEmpty => write!(f,"Token cannot be emtpy"),
            IdentityError::TokenIsInvalid => write!(f,"Token is invalid"),
            IdentityError::IssuerIsInvalid => write!(f,"Issuer is invalid"),
            IdentityError::SignatureHasExpired => write!(f,"Signature has expired"),
            IdentityError::SmtpDomainNotGood => write!(f,"Stmp domain is not good"),
            IdentityError::CouldNotSendEmail => write!(f,"Could not send the email throught the smtp transport"),
            IdentityError::CustomError(e) => write!(f,"{}",e)
        }
    }
}

impl Error for IdentityError { }