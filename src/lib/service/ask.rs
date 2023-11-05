use std::str::FromStr;
use crate::domain::clip::field;
use crate::ShortCode;

use derive_more::Constructor;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UpdateClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
    pub shortcode: field::ShortCode,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GetClip {
    pub shortcode: ShortCode,
    pub password: field::Password,
}

impl GetClip {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            shortcode: ShortCode::from(shortcode),
            password: field::Password::default(),
        }
    }
}

impl From<ShortCode> for GetClip {
    fn from(value: ShortCode) -> Self {
        Self {
            shortcode: value,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(value: &str) -> Self {
        Self::from_raw(value)
    }
}