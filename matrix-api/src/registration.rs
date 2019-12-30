use reqwest::StatusCode;
use serde_json::json;
use std::collections::HashMap;

use crate::api;
use crate::api::ApiError;
use crate::api::Result;
use crate::client::MatrixClient;

pub static ENDPOINT: &str = "/_matrix/client/r0/register";

#[derive(Deserialize, Debug)]
pub struct UserInteractiveAuthenticationModel {
   session: String,
   flows: Vec<UserAuthFlow>,
   params: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
pub struct UserAuthFlow {
   stages: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct Empty;

#[derive(Serialize, Debug)]
pub struct AuthModel {
   session: String,
   r#type: String,
}

#[derive(Serialize, Debug)]
pub enum RegistrationKind {
   Guest,
   User,
}

#[derive(Serialize, Debug)]
pub struct RegistrationModel {
   pub auth: AuthModel,
   pub kind: RegistrationKind,
   pub username: String,
   pub password: String,
   pub initial_device_display_name: String,
}

#[derive(Deserialize, Debug)]
pub struct RegistrationResponse {
   user_id: String,
   home_server: String,
   access_token: String,
   device_id: String,
}

pub fn auth_request(client: &MatrixClient) -> Result<UserInteractiveAuthenticationModel> {
   let mut auth_response = api::post(&client, ENDPOINT, &json!({}))?;

   let json = auth_response.json()?;

   Ok(json)
}

pub fn auth_select_flow(model: UserInteractiveAuthenticationModel) -> AuthModel {
   let flow = &model.flows[0];
   let stage = flow.stages[0].to_owned();

   let auth: AuthModel = AuthModel {
      session: model.session,
      r#type: stage,
   };

   println!(
      "{}",
      format!(
         "Session: {session}, Stage: {stage}",
         session = auth.session,
         stage = auth.r#type
      )
   );

   return auth;
}

pub fn register(client: &MatrixClient, model: RegistrationModel) -> Result<RegistrationResponse> {
   let mut response = api::post(client, ENDPOINT, &model)?;

   match response.status() {
      StatusCode::OK => {
         let success = response.json()?;
         Ok(success)
      }
      StatusCode::BAD_REQUEST
      | StatusCode::UNAUTHORIZED
      | StatusCode::FORBIDDEN
      | StatusCode::TOO_MANY_REQUESTS => Err(ApiError::from(response)),
      s => Err(ApiError::from(s)),
   }
}
