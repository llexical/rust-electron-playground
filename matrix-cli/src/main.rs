extern crate matrix_api;

mod io;
mod list_public_rooms;
mod login;
mod register;

use matrix_api::client::ApiClient;
pub static MATRIX_API_URL: &str = "http://my.matrix.host:8008";

fn request_action() -> String {
    println!("Select ation:");
    println!("- register (r)");
    println!("- login (l)");
    println!("- list public rooms (p)");
    let mut action = String::new();
    io::request_input("", &mut action);
    action
}

fn select_action(action: String) -> Result<(), matrix_api::api::ApiError> {
    match action.as_ref() {
        "r" => register::register_flow(),
        "l" => login::login_flow(),
        "p" => list_public_rooms::list_rooms(),
        _ => select_action(request_action()),
    }
}

fn continue_check() -> bool {
    println!("Continue? (y/n)");
    let mut action = String::new();
    io::request_input("", &mut action);
    match action.as_ref() {
        "Y" | "y" => true,
        "N" | "n" => false,
        _ => continue_check(),
    }
}

fn main() {
    loop {
        match select_action(request_action()) {
            Err(e) => println!("Error: {}", e),
            Ok(_) => println!("Success"),
        }
        if !continue_check() {
            println!("Goodbye!");
            break;
        }
        println!();
    }
}
