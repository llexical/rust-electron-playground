extern crate matrix_api;

use std::io;
use std::io::{Write};
use matrix_api::api::ApiError;
use matrix_api::*;

fn request_input(label: &str, buffer: &mut String) {
   print!("{}: ", label);
   io::stdout().flush().unwrap();
   match io::stdin().read_line(buffer) {
       Ok(_) => { *buffer = buffer.trim().to_string() },
       Err(_) => println!("failed to read {}", label),
   }
}

fn register_flow() -> Result<(), ApiError> {
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
        initial_device_display_name: String::from("cli")
    };
 
    registration::register(body)?;

    Ok(())
}

fn main() {
    match register_flow() {
        Err(e) => println!("Error: {}", e),
        Ok(_) => println!("Success")
    }
}

