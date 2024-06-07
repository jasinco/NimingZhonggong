pub mod error;
pub mod surreal;
pub mod test;

use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use totp_rs::{Secret, TOTP};

use self::error::{DBResult, RespondErr};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    password: String,
    verified: bool,
    school_id: String,
    email: String,
    totp: bool,
}

#[allow(dead_code)]
impl User {
    pub fn new(
        name: String,
        password: String,
        verified: bool,
        school_id: String,
        email: String,
        totp: bool,
    ) -> Self {
        Self {
            name,
            password,
            verified,
            school_id,
            email,
            totp,
        }
    }
    pub fn process_password(&mut self) -> DBResult<()> {
        self.password = hash(self.password.as_str(), DEFAULT_COST)?;
        Ok(())
    }
    pub fn verify_to(&self, another: Self) -> DBResult<()> {
        let result = verify(self.password.as_str(), another.password.as_str())?;
        if result {
            Ok(())
        } else {
            Err(RespondErr::Failed)
        }
    }
    pub fn fetch_id(&self) -> &str {
        self.school_id.as_str()
    }
    pub fn totp_enable(&mut self) {
        Totp2Fa::new(self);
        self.totp = true;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Totp2Fa {
    pub name: String,
    secret: String,
}
impl Totp2Fa {
    pub fn new(user: &User) -> Self {
        Self {
            name: user.name.clone(),
            secret: Secret::generate_secret().to_string(),
        }
    }
    fn totp_obj(&self) -> DBResult<TOTP> {
        Ok(TOTP::new(
            totp_rs::Algorithm::SHA1,
            6,
            1,
            30,
            Vec::from(self.secret.as_bytes()),
            Some("NimingZhongGong".to_string()),
            self.name.clone(),
        )?)
    }
    pub fn generate_totp_code(&self) -> DBResult<String> {
        let totp = self.totp_obj()?;
        Ok(totp.generate_current().expect("TimeError"))
    }
    pub fn generate_totp_url(&self) -> DBResult<String> {
        let totp = self.totp_obj()?;
        Ok(totp.get_url())
    }
}
