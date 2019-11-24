use serde_derive::{Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Deserialize, Serialize, Debug)]
pub enum AuthenticationTypes {
  #[serde(rename = "m.login.password")]
  Password,
  #[serde(rename = "m.login.recaptcha")]
  Captcha,
  #[serde(rename = "m.login.oauth2")]
  Oauth2,
  #[serde(rename = "m.login.email.identity")]
  Email,
  #[serde(rename = "m.login.msisdn")]
  Msisdn,
  #[serde(rename = "m.login.token")]
  Token,
  #[serde(rename = "m.login.dummy")]
  Dummy,
}

impl fmt::Display for AuthenticationTypes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
