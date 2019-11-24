extern crate matrix_api;

mod io;
mod login;
mod register;

fn request_action() -> String {
    println!("Select ation:");
    println!("- register (r)");
    println!("- login (l)");
    let mut action = String::new();
    io::request_input("", &mut action);
    action
}

fn select_action(action: String) -> Result<(), matrix_api::api::ApiError> {
    match action.as_ref() {
        "r" => register::register_flow(),
        "l" => login::login_flow(),
        _ => select_action(request_action()),
    }
}

fn main() {
    match select_action(request_action()) {
        Err(e) => println!("Error: {}", e),
        Ok(_) => println!("Success"),
    }
}
