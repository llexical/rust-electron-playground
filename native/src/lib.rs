#[macro_use]
extern crate neon;
extern crate matrix_api;
extern crate reqwest;

use matrix_api::api::ApiError;
use matrix_api::client::MatrixClient;
use matrix_api::*;
use neon::prelude::*;

pub static MATRIX_API_URL: &str = "http://my.matrix.host:8008";

fn register_flow(username: String, password: String) -> Result<(), matrix_api::api::ApiError> {
    let matrix_client = MatrixClient::new(MATRIX_API_URL);
    let interactive_auth_model = registration::auth_request(&matrix_client)?;

    let auth = registration::auth_select_flow(interactive_auth_model);

    println!("{}, {}", username, password);

    let body = registration::RegistrationModel {
        auth,
        kind: registration::RegistrationKind::User,
        username,
        password,
        initial_device_display_name: String::from("cli"),
    };

    registration::register(&matrix_client, body)?;

    Ok(())
}

fn cx_response(
    mut cx: FunctionContext,
    success: bool,
    message: String,
) -> (FunctionContext, Handle<JsObject>) {
    let success = cx.boolean(success);
    let message = cx.string(message);

    let response_obj = cx.empty_object();
    response_obj.set(&mut cx, "success", success).unwrap();
    response_obj.set(&mut cx, "message", message).unwrap();
    (cx, response_obj)
}

fn register_user(mut cx: FunctionContext) -> JsResult<JsObject> {
    let username = cx.argument::<JsString>(0)?.value();
    let password = cx.argument::<JsString>(1)?.value();

    match register_flow(username, password) {
        Err(e) => {
            let (mut cx, response) = match e {
                ApiError::Network { kind: _, message } => cx_response(cx, false, message),
                ApiError::Http(status_code, message) => match message {
                    Some(m) => cx_response(cx, false, String::from(m)),
                    None => cx_response(cx, false, status_code.to_string()),
                },
                ApiError::Response(_, m) => cx_response(cx, false, m.message),
                ApiError::Serialization => cx_response(
                    cx,
                    false,
                    String::from("There was a serialization error m8."),
                ),
                ApiError::Unknown => cx_response(
                    cx,
                    false,
                    String::from("This error is unknown, please panic."),
                ),
            };
            return cx.throw(response);
        }
        Ok(_) => Ok(cx_response(cx, true, format!("")).1),
    }
}

register_module!(mut cx, {
    cx.export_function("register_user", register_user)
});
