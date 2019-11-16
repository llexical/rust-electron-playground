#[macro_use]
extern crate neon;
extern crate matrix_api;
extern crate reqwest;

use neon::prelude::*;

fn register_flow(username: String, password: String) -> Result<(), matrix_api::api::ApiError> {
    let interactive_auth_model = matrix_api::registration::auth_request()?;

    let auth = matrix_api::registration::auth_select_flow(interactive_auth_model);

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
            let (mut cx, response) = cx_response(cx, false, format!("{}", e));
            return cx.throw(response);
        }
        Ok(_) => Ok(cx_response(cx, true, format!("")).1),
    }
}

register_module!(mut cx, {
    cx.export_function("register_user", register_user)
});
