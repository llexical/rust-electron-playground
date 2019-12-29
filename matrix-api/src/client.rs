struct ApiClient {
  base_url: String,
  access_token: Option<String>,
}

impl ApiClient {
  // Create a new ApiClient with base url
  fn new(base_url: String) -> ApiClient {
    ApiClient {
      base_url: base_url,
      access_token: None,
    }
  }

  // Set the users access token (e.g. login)
  fn set_access_token(&mut self, access_token: String) {
    self.access_token = Some(access_token);
  }

  // Remove the users access token (e.g. logout)
  fn remove_access_token(&mut self) {
    self.access_token = None;
  }
}
