use actix_web::web::Bytes;
use bcrypt::BcryptError;
use derive_more::{Display, Error};
use totp_rs::{SecretParseError, TotpUrlError};

pub type DBResult<T> = Result<T, RespondErr>;

#[derive(Error, Display, Debug)]
pub enum RespondErr {
    Failed,
    Surreal(surrealdb::Error),
    Bcrypt(BcryptError),
    TOTPERR(TotpUrlError),
    SecretParse(SecretParseError),
    Empty,
    Existed,
}

impl From<surrealdb::Error> for RespondErr {
    fn from(err: surrealdb::Error) -> RespondErr {
        RespondErr::Surreal(err)
    }
}
impl From<BcryptError> for RespondErr {
    fn from(err: BcryptError) -> RespondErr {
        RespondErr::Bcrypt(err)
    }
}
impl From<TotpUrlError> for RespondErr {
    fn from(err: TotpUrlError) -> RespondErr {
        RespondErr::TOTPERR(err)
    }
}
impl From<SecretParseError> for RespondErr {
    fn from(err: SecretParseError) -> RespondErr {
        RespondErr::SecretParse(err)
    }
}
