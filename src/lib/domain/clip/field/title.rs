use std::str::FromStr;
use rocket::form::{FromFormField,self, ValueField};
use serde::{Deserialize, Serialize};

use crate::domain::clip::ClipError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T>(title: T) -> Self
        where T: Into<Option<String>> {
        let password: Option<String> = title.into();

        match password {
            None => Self(None),
            Some(password) => {
                if !password.trim().is_empty() {
                    Self(Some(password))
                } else {
                    Self(None)
                }
            }
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_string()))
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value.to_owned()))
    }
}