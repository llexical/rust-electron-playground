use reqwest::StatusCode;
use serde_derive::{Deserialize, Serialize};

use crate::api;
use crate::api::ApiError;
use crate::api::Result;
use crate::client::MatrixClient;

pub static ENDPOINT: &str = "/_matrix/client/r0/publicRooms";

#[derive(Serialize, Debug)]
pub struct PublicRoomsQuery {
  pub limit: i64,
  pub since: Option<String>,
  pub server: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct PublicRoomsRequest {}

#[derive(Deserialize, Debug)]
pub struct PublicRoomsResponse {
  pub chunk: Vec<PublicRoomsChunk>,
  pub next_batch: Option<String>,
  pub prev_batch: Option<String>,
  pub total_room_count_estimate: i64,
}

#[derive(Deserialize, Debug)]
pub struct PublicRoomsChunk {
  pub aliases: Vec<String>,
  pub canonical_alias: String,
  pub name: String,
  pub num_joined_members: i64,
  pub room_id: String,
  pub topic: String,
  pub world_readable: bool,
  pub guest_can_join: bool,
  pub avatar_url: Option<String>,
}

pub fn list_public_rooms(
  client: &MatrixClient,
  query: PublicRoomsQuery,
) -> Result<PublicRoomsResponse> {
  let mut response = api::get_query(&client, ENDPOINT, &query)?;

  match response.status() {
    StatusCode::OK => {
      let success = response.json()?;
      Ok(success)
    }
    s => Err(ApiError::from(s)),
  }
}

pub fn filter_public_rooms(
  client: &MatrixClient,
  query: PublicRoomsQuery,
  request: PublicRoomsRequest,
) -> Result<PublicRoomsResponse> {
  let mut response = api::post_query(&client, ENDPOINT, &request, &query)?;

  match response.status() {
    StatusCode::OK => {
      let success = response.json()?;
      Ok(success)
    }
    s => Err(ApiError::from(s)),
  }
}
