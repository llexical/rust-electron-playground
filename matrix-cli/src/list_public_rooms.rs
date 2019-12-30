use matrix_api::api::ApiError;
use matrix_api::client::MatrixClient;
use matrix_api::*;

pub fn list_rooms(matrix_client: &MatrixClient) -> Result<(), ApiError> {
  let query = rooms::public::PublicRoomsQuery {
    limit: 1000,
    server: Option::None,
    since: Option::None,
  };
  let response = rooms::public::list_public_rooms(&matrix_client, query)?;

  // Test to see if matrix client has set the access token
  match &matrix_client.access_token {
    Some(access_token) => println!("Access Token: {}", access_token),
    None => println!("No access token"),
  }

  println!("total count: {}", response.total_room_count_estimate);

  for room in response.chunk {
    println!("----");
    println!("Alias: {}", room.canonical_alias);
    println!("Name: {}", room.name);
    println!("Members: {}", room.num_joined_members);
    println!("Room ID: {}", room.room_id);
    println!("----");
  }

  Ok(())
}
