use serde_json::Value;
use std::collections::HashMap;
use std::error;
use std::fmt;

use crate::client::MatrixClient;

#[derive(Deserialize, Debug, Clone)]
pub enum Kind {
  Http,
  Redirect,
  Timeout,
}

#[derive(Deserialize, Debug, Clone)]
pub enum MatrixErrorCode {
  #[serde(rename = "M_FORBIDDEN")]
  Forbidden,
  #[serde(rename = "M_UNKNOWN_TOKEN")]
  UnknownToken,
  #[serde(rename = "M_MISSING_TOKEN")]
  MissingToken,
  #[serde(rename = "M_BAD_JSON")]
  BadJson,
  #[serde(rename = "M_NOT_JSON")]
  NotJson,
  #[serde(rename = "M_NOT_FOUND")]
  NotFound,
  #[serde(rename = "M_LIMIT_EXCEEDED")]
  LimitExceeded,
  #[serde(rename = "M_UNKNOWN")]
  Unknown,
  #[serde(rename = "M_UNRECOGNIZED")]
  Unrecognized,
  #[serde(rename = "M_UNAUTHORIZED")]
  Unauthorised,
  #[serde(rename = "M_USER_DEACTIVATED")]
  UserDeactivated,
  #[serde(rename = "M_USER_IN_USE")]
  UserInUse,
  #[serde(rename = "M_INVALID_USERNAME")]
  InvalidUsername,
  #[serde(rename = "M_ROOM_IN_USE")]
  RoomInUse,
  #[serde(rename = "M_INVALID_ROOM_STATE")]
  InvalidRoomState,
  #[serde(rename = "M_THREEPID_IN_USE")]
  ThreepidInUse,
  #[serde(rename = "M_THREEPID_NOT_FOUND")]
  ThreepidNotFound,
  #[serde(rename = "M_THREEPID_AUTH_FAILED")]
  ThreepidAuthFailed,
  #[serde(rename = "M_THREEPID_DENIED")]
  ThreepidDenied,
  #[serde(rename = "M_SERVER_NOT_TRUSTED")]
  ServerNotTrusted,
  #[serde(rename = "M_UNSUPPORTED_ROOM_VERSION")]
  UnsupportedRoomVersion,
  #[serde(rename = "M_INCOMPATIBLE_ROOM_VERSION")]
  IncompatibleRoomVersion,
  #[serde(rename = "M_BAD_STATE")]
  BadState,
  #[serde(rename = "M_GUEST_ACCESS_FORBIDDEN")]
  GuestAccessForbidden,
  #[serde(rename = "M_CAPTCHA_NEEDED")]
  CaptchaNeeded,
  #[serde(rename = "M_CAPTCHA_INVALID")]
  CaptchaInvalid,
  #[serde(rename = "M_MISSING_PARAM")]
  MissingParam,
  #[serde(rename = "M_INVALID_PARAM")]
  InvalidParam,
  #[serde(rename = "M_TOO_LARGE")]
  TooLarge,
  #[serde(rename = "M_EXCLUSIVE")]
  Exclusive,
  #[serde(rename = "M_RESOURCE_LIMIT_EXCEEDED")]
  ResourceLimitExceeded,
  #[serde(rename = "M_CANNOT_LEAVE_SERVER_NOTICE_ROOM")]
  CannotLeaveServerNoticeRoom,
}

#[derive(Deserialize, Debug)]
pub struct MatrixErrorResponse {
  #[serde(rename = "errcode")]
  pub code: MatrixErrorCode,
  #[serde(rename = "error")]
  pub message: String,
  #[serde(flatten)]
  params: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiError {
  Network { kind: Kind, message: String },
  Http(u16, Option<&'static str>),
  Response(u16, MatrixErrorResponse),
  Serialization,
  Unknown,
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for crate::api::ApiError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ApiError::Network {
        kind: _,
        ref message,
      } => write!(f, "Network error occurred: {}", message),
      ApiError::Http(code, _) => write!(f, "Http error: {}", code),
      ApiError::Response(_, r) => write!(f, "Response error: {}", r.message),
      ApiError::Serialization => write!(f, "Serialization error occured"),
      ApiError::Unknown => write!(f, "Unknown error occured"),
    }
  }
}

// This is important for other errors to wrap this one.
impl error::Error for ApiError {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    // Generic error, underlying cause isn't tracked.
    None
  }
}

impl From<reqwest::Error> for ApiError {
  fn from(error: reqwest::Error) -> ApiError {
    if error.is_http() {
      ApiError::Network {
        kind: Kind::Http,
        message: String::from("HTTP error occured"),
      }
    } else if error.is_redirect() {
      ApiError::Network {
        kind: Kind::Redirect,
        message: String::from("Redirect loop"),
      }
    } else if error.is_timeout() {
      ApiError::Network {
        kind: Kind::Timeout,
        message: String::from("Network request timed out"),
      }
    } else if error.is_serialization() {
      let serde_error = match error.get_ref() {
        None => return ApiError::Unknown,
        Some(err) => err,
      };
      println!("problem parsing information {}", serde_error);
      ApiError::Serialization
    } else {
      ApiError::Unknown
    }
  }
}

impl From<reqwest::StatusCode> for ApiError {
  fn from(status: reqwest::StatusCode) -> ApiError {
    ApiError::Http(status.as_u16(), status.canonical_reason())
  }
}

impl From<reqwest::Response> for ApiError {
  fn from(mut response: reqwest::Response) -> ApiError {
    match response.json() {
      Ok(error) => ApiError::Response(response.status().as_u16(), error),
      Err(_) => ApiError::from(response.status()),
    }
  }
}

pub type Result<T> = ::std::result::Result<T, ApiError>;

pub fn post<TBody: serde::Serialize + ?Sized>(
  api_client: &MatrixClient,
  endpoint: &str,
  body: &TBody,
) -> Result<reqwest::Response> {
  let client = reqwest::Client::new();
  let url = format!("{}{}", api_client.get_base_url(), endpoint);
  let response = client.post(&url).json(body).send()?;

  Ok(response)
}

pub fn post_query<TBody: serde::Serialize + ?Sized, TQuery: serde::Serialize + ?Sized>(
  api_client: &MatrixClient,
  endpoint: &str,
  body: &TBody,
  query: &TQuery,
) -> Result<reqwest::Response> {
  let client = reqwest::Client::new();
  let url = format!("{}{}", api_client.get_base_url(), endpoint);
  let response = client.post(&url).query(query).json(body).send()?;

  Ok(response)
}

pub fn get(api_client: &MatrixClient, endpoint: &str) -> Result<reqwest::Response> {
  let client = reqwest::Client::new();
  let url = format!("{}{}", api_client.get_base_url(), endpoint);
  let response = client.get(&url).send()?;

  Ok(response)
}

pub fn get_query<TQuery: serde::Serialize + ?Sized>(
  api_client: &MatrixClient,
  endpoint: &str,
  model: &TQuery,
) -> Result<reqwest::Response> {
  let client = reqwest::Client::new();
  let url = format!("{}{}", api_client.get_base_url(), endpoint);
  let response = client.get(&url).query(model).send()?;

  Ok(response)
}
