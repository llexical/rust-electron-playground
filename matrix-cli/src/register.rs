use matrix_api::api::ApiError;
use matrix_api::*;

use crate::io::request_input;

pub fn register_flow() -> Result<(), ApiError> {
  let interactive_auth_model = registration::auth_request()?;

  println!("step 1");
  let auth = registration::auth_select_flow(interactive_auth_model);

  let mut username = String::new();
  request_input("Username", &mut username);

  let mut password = String::new();
  request_input("Password", &mut password);

  println!("{}, {}", username, password);

  let body = registration::RegistrationModel {
    auth,
    kind: registration::RegistrationKind::User,
    username,
    password,
    initial_device_display_name: String::from("cli"),
  };

  registration::register(body)?;

  Ok(())
}
