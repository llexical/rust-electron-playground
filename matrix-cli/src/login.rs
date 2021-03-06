use matrix_api::api::ApiError;
use matrix_api::client::MatrixClient;
use matrix_api::*;

use crate::io::request_input;

fn select_flow(flow_count: usize) -> usize {
  let mut selected_flow = String::new();
  request_input("Select flow", &mut selected_flow);

  match selected_flow.parse::<usize>() {
    Ok(i) => {
      if i > flow_count {
        println!("invalid selection");
        return select_flow(flow_count);
      }
      i
    }
    Err(_) => {
      println!("invalid selection");
      select_flow(flow_count)
    }
  }
}

fn login_password(matrix_client: &mut MatrixClient) -> Result<(), ApiError> {
  let mut username = String::new();
  request_input("Username", &mut username);
  let identifier = login::UserIdentifier::User { user: username };

  let mut password = String::new();
  request_input("Password", &mut password);
  let login_type = login::LoginType::Password { password: password };

  let body = login::LoginModel {
    r#type: login_type,
    identifier,
    device_id: String::from("0001"),
    initial_device_display_name: String::from("cli"),
  };

  let response = login::login(&matrix_client, body)?;
  matrix_client.set_access_token(response.access_token);

  Ok(())
}

pub fn login_flow(matrix_client: &mut MatrixClient) -> Result<(), ApiError> {
  let flows = login::get_login_flows(&matrix_client)?;
  let flow_count = flows.flows.len();
  println!("Available flows: ");
  for i in 0..flow_count {
    println!("({}) - {}", i, flows.flows[i].r#type);
  }

  let selected_flow = &flows.flows[select_flow(flow_count)].r#type;

  match selected_flow {
    auth::AuthenticationTypes::Password => login_password(matrix_client)?,
    _ => {
      println!("Unsupported");
      login_flow(matrix_client)?
    }
  };

  Ok(())
}
