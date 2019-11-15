#[macro_use]
extern crate neon;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use neon::prelude::*;
use reqwest::Client;

#[derive(Deserialize, Debug)]
struct RegisterAuthResponse {
    session: String,
}

#[derive(Deserialize, Debug)]
struct RegisterResponse {
    user_id: String,
    home_server: String,
    access_token: String,
    device_id: String,
}

fn register_user(mut cx: FunctionContext) -> JsResult<JsObject> {
    let username = cx.argument::<JsString>(0)?.value();
    let password = cx.argument::<JsString>(1)?.value();

    let session_token = match make_auth_request() {
        Err(e) => {
            let success = cx.boolean(false);
            let error_msg = cx.string(handler(e));

            let response_obj = cx.empty_object();
            response_obj.set(&mut cx, "success", success)?;
            response_obj.set(&mut cx, "error_msg", error_msg)?;

            return cx.throw(response_obj);
        }
        Ok(response) => response.session,
    };

    let _register_response = match make_register_request(username, password, session_token) {
        Err(e) => {
            let success = cx.boolean(false);
            let error_msg = cx.string(handler(e));

            let response_obj = cx.empty_object();
            response_obj.set(&mut cx, "success", success)?;
            response_obj.set(&mut cx, "error_msg", error_msg)?;

            return cx.throw(response_obj);
        }
        Ok(response) => response,
    };

    let success = cx.boolean(true);
    let response_obj = cx.empty_object();
    response_obj.set(&mut cx, "success", success)?;

    return Ok(response_obj);
}
// Response is not a json object conforming to the Simple struct
fn make_auth_request() -> Result<RegisterAuthResponse, reqwest::Error> {
    let request_url = "http://my.matrix.host:8008/_matrix/client/r0/register?kind=user";
    let request_body = json!({});

    return Client::new()
        .post(request_url)
        .json(&request_body)
        .send()?
        .json();
}

fn make_register_request(
    username: String,
    password: String,
    session_token: String,
) -> Result<RegisterResponse, reqwest::Error> {
    let request_url = "http://my.matrix.host:8008/_matrix/client/r0/register?kind=user";
    let request_body = json!({
      "auth": {
        "session": session_token,
        "type": "m.login.dummy"
      },
      "username": username,
      "password": password,
      "initial_device_display_name": "rustapp"
    });

    return Client::new().post(request_url).json(&request_body).send()?.json();
}

fn handler(e: reqwest::Error) -> String {
    if e.is_http() {
        match e.url() {
            None => return format!("No Url given"),
            Some(url) => return format!("Problem making request to: {}", url),
        }
    }
    if e.is_client_error() {
        return format!(
            "There was a {} error making the request to the server",
            e.status().unwrap()
        );
    }
    if e.is_server_error() {
        return format!("There was an {} error from the server", e.status().unwrap());
    }
    // Inspect the internal error and output it
    if e.is_serialization() {
        match e.get_ref() {
            None => return format!("problem parsing information"),
            Some(err) => return format!("problem parsing information {}", err),
        };
    }
    if e.is_redirect() {
        return format!("server redirecting too many times or making loop");
    }

    return format!("There was an error making your request");
}

register_module!(mut cx, {
    cx.export_function("register_user", register_user)
});
