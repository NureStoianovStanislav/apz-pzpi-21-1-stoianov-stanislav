use serde::Serialize;

use crate::Error;

pub type UnvalidatedEmail = String;

#[derive(Clone, Debug, sqlx::Type, Serialize)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Email(UnvalidatedEmail);

impl Email {
    pub fn new(value: UnvalidatedEmail) -> crate::Result<Email> {
        if validate_email(&value) {
            Ok(Self(value))
        } else {
            Err(Error::Validation("unsupported email address"))
        }
    }
}

impl TryFrom<UnvalidatedEmail> for Email {
    type Error = Error;

    fn try_from(value: UnvalidatedEmail) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

fn validate_email(email: &str) -> bool {
    if email.len() > 50 {
        return false;
    }
    let Some((username, domain)) = email.split_once('@') else {
        return false;
    };
    if !domain.contains('.') {
        return false;
    }
    for part in [username, domain] {
        let is_invalid_character = |c| match c {
            c if c.is_alphanumeric() => false,
            '.' | '-' | '_' => false,
            _ => true,
        };
        if part.contains("..") || part.chars().any(is_invalid_character) {
            return false;
        }
    }
    true
}
