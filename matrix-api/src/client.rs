pub struct MatrixClient {
  pub base_url: String,
  pub access_token: Option<String>,
}

impl MatrixClient {
  // Create a new MatrixClient with base url
  pub fn new(base_url: &str) -> MatrixClient {
    MatrixClient {
      base_url: base_url.to_string(),
      access_token: None,
    }
  }

  pub fn get_base_url(&self) -> &String {
    &self.base_url
  }

  // Set the users access token (e.g. login)
  pub fn set_access_token(&mut self, access_token: String) {
    self.access_token = Some(format!("Bearer {}", access_token));
  }

  // Return access token as an Option
  pub fn get_access_token(&self) -> &Option<String> {
    &self.access_token
  }

  // Remove the users access token (e.g. logout)
  pub fn remove_access_token(&mut self) {
    self.access_token = None;
  }
}
