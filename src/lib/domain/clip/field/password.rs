use std::str::FromStr;

use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

use crate::domain::clip::ClipError;

#[derive(Clone, Debug, Deserialize, Serialize, PartialOrd, PartialEq)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T>(password: T) -> Result<Self, ClipError>
        where T: Into<Option<String>> {
        let password: Option<String> = password.into();

        match password {
            None => Ok(Self(None)),
            Some(password) => {
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                } else {
                    Ok(Self(None))
                }
            }
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Password {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(
            Self::new(field.value.to_owned())
                .map_err(|e| form::Error::validation(format!("{}", e)))?
        )
    }
}