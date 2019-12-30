use matrix_api::api::ApiError;
use matrix_api::client::MatrixClient;
use matrix_api::*;

use crate::io::request_input;

pub fn create(matrix_client: &MatrixClient) -> Result<(), ApiError> {
  let mut room_name = String::new();
  request_input("Room Name (e.g. General Fun)", &mut room_name);

  let mut room_name_alias = String::new();
  request_input("Room Name Alias (e.g. general-fun)", &mut room_name_alias);

  let mut room_topic = String::new();
  request_input("Room Topic (e.g. To have fun)", &mut room_topic);

  let request = rooms::create::CreateRoomRequest {
    visibility: Some(rooms::create::VisibilityType::Public),
    room_alias_name: Some(room_name_alias),
    name: Some(room_name),
    topic: Some(room_topic),
    invite: None,
    invite_3pid: None,
    room_version: None,
    creation_content: Some(rooms::create::CreationContent {
      federate: Some(false),
      predecessor: None,
    }),
    initial_state: None,
    preset: Some(rooms::create::PresetType::PublicChat),
    is_direct: Some(false),
    power_level_content_override: None,
  };

  rooms::create::create_room(&matrix_client, request)?;

  Ok(())
}
