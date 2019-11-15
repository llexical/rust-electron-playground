use std::error;
use std::fmt;

#[derive(Deserialize, Debug, Clone)]
pub enum Kind {
  Http,
  Redirect,
  Timeout
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
  CannotLeaveServerNoticeRoom
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiError {
  Network {
     kind: Kind,
     message: String
  },
  Response {
    #[serde(rename = "errcode")]
    code: MatrixErrorCode,
    #[serde(rename = "error")]
    message: String,
  },
  Serialization,
  Unknown
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for crate::api::ApiError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        ApiError::Network { kind: _, ref message } => write!(f, "Network error occurred: {}", message),
        ApiError::Response { code: _, ref message } => write!(f, "Response error: {}", message),
        ApiError::Serialization => write!(f, "Serialization error occured"),
        ApiError::Unknown => write!(f, "Unknown error occured")
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
        message: String::from("HTTP error occured")
      }
    } else if error.is_redirect() {
      ApiError::Network {
        kind: Kind::Redirect,
        message: String::from("Redirect loop")
      }
    } else if error.is_timeout() {
      ApiError::Network {
        kind: Kind::Timeout,
        message: String::from("Network request timed out")
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

pub type Result<T> = ::std::result::Result<T, ApiError>;

pub fn post<TModel: serde::Serialize + ?Sized>(url: &str, model: &TModel) -> Result<reqwest::Response> {
  let client = reqwest::Client::new();
  let response = client.post(url)
       .json(model)
       .send()?;

  Ok(response)
}