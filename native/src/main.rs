extern crate matrix_api;
extern crate reqwest;
use std::io;
use std::io::Write;

fn request_input(label: &str, buffer: &mut String) {
  print!("{}: ", label);
  io::stdout().flush().unwrap();
  match io::stdin().read_line(buffer) {
    Ok(_) => *buffer = buffer.trim().to_string(),
    Err(_) => println!("failed to read {}", label),
  }
}

fn register_flow() -> Result<(), matrix_api::api::ApiError> {
  let interactive_auth_model = matrix_api::registration::auth_request()?;

  println!("step 1");
  let auth = matrix_api::registration::auth_select_flow(interactive_auth_model);

  let mut username = String::new();
  request_input("Username", &mut username);

  let mut password = String::new();
  request_input("Password", &mut password);

  println!("{}, {}", username, password);

  let body = matrix_api::registration::RegistrationModel {
    auth,
    kind: matrix_api::registration::RegistrationKind::User,
    username,
    password,
    initial_device_display_name: String::from("cli"),
  };

  matrix_api::registration::register(body)?;

  Ok(())
}

fn main() {
  match register_flow() {
    Err(e) => println!("Error: {}", e),
    Ok(_) => println!("Success"),
  }
}
