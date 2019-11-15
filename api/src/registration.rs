use reqwest::{StatusCode};
use std::collections::HashMap;
use serde_json::json;

use crate::api;

pub static API_URL: &str = "http://my.matrix.host:8008/_matrix/client/r0/register";

#[derive(Deserialize, Debug)]
pub struct UserInteractiveAuthenticationModel {
   session: String,
   flows: Vec<UserAuthFlow>,
   params: HashMap<String, String>
}

#[derive(Deserialize, Debug)]
pub struct UserAuthFlow {
   stages: Vec<String>
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
   User
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
   device_id: String
}

pub fn auth_request() -> Result<UserInteractiveAuthenticationModel, api::ApiError> {
   let mut auth_response = api::post(API_URL, &json!({}))?;

   let json = auth_response.json()?;

   Ok(json)
}

pub fn auth_select_flow(model: UserInteractiveAuthenticationModel) -> AuthModel {
   let flow = &model.flows[0];
   let stage = flow.stages[0].to_owned();

   let auth: AuthModel = AuthModel {
       session: model.session,
       r#type: stage
   };

   println!("{}", format!(
       "Session: {session}, Stage: {stage}",
       session = auth.session,
       stage = auth.r#type
   ));

   return auth;
}

pub fn register(model: RegistrationModel) -> Result<RegistrationResponse, api::ApiError> {
   let mut response = api::post(API_URL, &model)?;

   match response.status() {
       StatusCode::OK => {
           let success = response.json()?;
           Ok(success)
       },
       StatusCode::BAD_REQUEST => {
           let error : api::ApiError = response.json()?;
           Err(error)
       },
       s => {
          println!("Received response status: {:?}", s);
          Err(api::ApiError::Unknown)
       }
   }
}