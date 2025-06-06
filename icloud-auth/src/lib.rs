pub mod anisette;
mod client;
use std::fmt::Display;

pub use client::{AppleAccount, LoginState, TrustedPhoneNumber, AuthenticationExtras, VerifyBody, CircleSendMessage};
pub use omnisette::{LoginClientInfo, ArcAnisetteClient};

use thiserror::Error;
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse the response")]
    Parse,
    #[error("Failed to authenticate.")]
    AuthSrp,
    #[error("Failed to have a happy birthday!.")]
    HappyBirthdayError,
    #[error("Bad 2fa code.")]
    Bad2faCode,
    #[error("{1} ({0})")]
    AuthSrpWithMessage(i64, String),
    #[error("Please login to appleid.apple.com to fix this account")]
    ExtraStep(String),
    #[error("Failed to parse a plist {0}")]
    PlistError(#[from] plist::Error),
    #[error("Request failed {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed getting anisette data {0}")]
    ErrorGettingAnisette(#[from] omnisette::AnisetteError)
}