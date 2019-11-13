use actix_web::HttpRequest;

use generust_example_project_service::ctx::Router;

use generust_example_project_core::{Error, Result};

#[derive(derive_more::Constructor, Debug)]
pub(crate) struct RequestRouter {
  req: HttpRequest
}

impl Router for RequestRouter {
  fn route(&self, path: &str, args: &[&str]) -> Result<String> {
    self
      .req
      .url_for(path, args)
      .map(|x| x.path().into())
      .map_err(|_| Error::from(format!("Unable to find route for [{}]", path)))
  }
}
