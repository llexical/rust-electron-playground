use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};

use crate::api;
use crate::api::ApiError;
use crate::api::Result;
use crate::auth::AuthenticationTypes;

pub static API_URL: &str = "http://my.matrix.host:8008/_matrix/client/r0/login";

#[derive(Deserialize, Debug)]
pub struct LoginFlows {
  pub flows: Vec<LoginFlow>,
}

#[derive(Deserialize, Debug)]
pub struct LoginFlow {
  pub r#type: AuthenticationTypes,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum LoginType {
  #[serde(rename = "m.login.password")]
  Password { password: String },
  #[serde(rename = "m.login.token")]
  Token { token: String },
}

#[derive(Serialize, Debug)]
pub enum ThirdPartyMedium {
  #[serde(rename = "email")]
  Email,
  #[serde(rename = "msisdn")]
  MSISDN,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum UserIdentifier {
  #[serde(rename = "m.id.user")]
  User { user: String },
  #[serde(rename = "m.id.thirdparty")]
  ThirdParty {
    medium: ThirdPartyMedium,
    address: String,
  },
  #[serde(rename = "m.id.phone")]
  Phone { country: String, phone: String },
}

#[derive(Serialize, Debug)]
pub struct LoginModel {
  #[serde(flatten, rename = "type")]
  pub r#type: LoginType,
  pub identifier: UserIdentifier,
  pub device_id: String,
  pub initial_device_display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct ServerInformation {
  pub base_url: String,
}

#[derive(Deserialize, Debug)]
pub struct DiscoveryInformation {
  #[serde(rename = "m.homeserver")]
  pub homeserver: ServerInformation,
  #[serde(rename = "m.identity_server")]
  pub identity_server: ServerInformation,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
  pub user_id: String,
  pub access_token: String,
  pub home_server: String,
  pub device_id: String,
  pub well_known: Option<DiscoveryInformation>,
}

pub fn get_login_flows() -> Result<LoginFlows> {
  let mut response = api::get(API_URL)?;

  match response.status() {
    StatusCode::OK => {
      let success = response.json()?;
      Ok(success)
    }
    StatusCode::TOO_MANY_REQUESTS => Err(ApiError::from(response)),
    s => Err(ApiError::from(s)),
  }
}

pub fn login(model: LoginModel) -> Result<LoginResponse> {
  let mut response = api::post(API_URL, &model)?;

  match response.status() {
    StatusCode::OK => {
      let success = response.json()?;
      Ok(success)
    }
    StatusCode::BAD_REQUEST | StatusCode::FORBIDDEN | StatusCode::TOO_MANY_REQUESTS => {
      Err(ApiError::from(response))
    }
    s => Err(ApiError::from(s)),
  }
}
