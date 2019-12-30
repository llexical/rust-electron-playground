use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api;
use crate::api::ApiError;
use crate::api::Result;
use crate::client::MatrixClient;

/*
Create Room
API for creating a room

docs: https://matrix.org/docs/spec/client_server/latest#post-matrix-client-r0-createroom
*/

pub static ENDPOINT: &str = "/_matrix/client/r0/createRoom";

#[derive(Serialize, Debug)]
pub enum VisibilityType {
  #[serde(rename = "public")]
  Public,
  #[serde(rename = "private")]
  Private,
}

#[derive(Serialize, Debug)]
pub struct Invite3pid {
  pub id_server: String,
  pub id_access_token: String,
  pub medium: String,
  pub address: String,
}

#[derive(Serialize, Debug)]
pub struct PreviousRoom {
  pub room_id: String,
  pub event_id: String,
}

#[derive(Serialize, Debug)]
pub struct CreationContent {
  #[serde(rename = "m.federate")]
  pub federate: Option<bool>,
  pub predecessor: Option<PreviousRoom>,
}

#[derive(Serialize, Debug)]
pub struct StateEvent {
  pub r#type: String,
  pub state_key: Option<String>,
  // TODO: Incorrect type definition, need to confirm what 'object' is
  pub content: String,
}

#[derive(Serialize, Debug)]
pub enum PresetType {
  #[serde(rename = "private_chat")]
  PrivateChat,
  #[serde(rename = "trusted_private_chat")]
  TrustedPrivateChat,
  #[serde(rename = "public_chat")]
  PublicChat,
}

#[derive(Serialize, Debug)]
pub struct Notifications {
  pub room: u16,
}

// https://matrix.org/docs/spec/client_server/latest#m-room-power-levels
#[derive(Serialize, Debug)]
pub struct PowerLevels {
  pub ban: Option<u16>,
  // Mapping from event types to power level required
  // TODO: Make more specific (limit to matrix event types)
  pub events: Option<HashMap<String, u16>>,
  pub events_default: Option<u16>,
  pub invite: Option<u16>,
  pub kick: Option<u16>,
  pub redact: Option<u16>,
  pub state_default: Option<u16>,
  // Mapping from user id's to power level for each user
  pub users: Option<HashMap<String, u16>>,
  pub users_default: Option<u16>,
  pub notifications: Option<Notifications>,
}

#[derive(Serialize, Debug)]
pub struct CreateRoomRequest {
  pub visibility: Option<VisibilityType>,
  pub room_alias_name: Option<String>,
  pub name: Option<String>,
  pub topic: Option<String>,
  pub invite: Option<Vec<String>>,
  pub invite_3pid: Option<Vec<Invite3pid>>,
  pub room_version: Option<String>,
  pub creation_content: Option<CreationContent>,
  pub initial_state: Option<Vec<StateEvent>>,
  pub preset: Option<PresetType>,
  pub is_direct: Option<bool>,
  pub power_level_content_override: Option<PowerLevels>,
}

#[derive(Deserialize, Debug)]
pub struct CreateRoomResponse {
  room_id: String,
}

pub fn create_room(
  client: &MatrixClient,
  request: CreateRoomRequest,
) -> Result<CreateRoomResponse> {
  let mut response = api::post(&client, ENDPOINT, &request)?;

  match response.status() {
    StatusCode::OK => {
      let success = response.json()?;
      Ok(success)
    }
    StatusCode::BAD_REQUEST | StatusCode::FORBIDDEN | StatusCode::UNAUTHORIZED => {
      Err(ApiError::from(response))
    }
    s => Err(ApiError::from(s)),
  }
}
