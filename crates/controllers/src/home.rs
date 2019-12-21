use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::forms::profile_form::ProfileForm;

use generust_example_project_core::profile::UserProfile;
use generust_example_project_service::AppConfig;

/// Available at `/`
pub fn index(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    generust_example_project_templates::home::index(ctx, router)
  })
}

/// Available at `/health`
pub fn health() -> HttpResponse {
  HttpResponse::Ok().finish()
}

/// Available at `/profile`
pub fn profile(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    generust_example_project_templates::profile::profile(ctx, router)
  })
}

/// Available by posting to `/profile`
pub fn profile_post(session: Session, cfg: web::Data<AppConfig>, req: HttpRequest, form: Option<web::Form<ProfileForm>>) -> HttpResponse {
  match form {
    Some(f) => crate::redir(&session, &cfg, req, |ctx, router| {
      let profile = UserProfile::new(
        String::from(f.username()),
        f.theme(),
        f.navbar_color().into(),
        f.link_color().into()
      );
      generust_example_project_service::profile::save(cfg.files(), ctx.user_id(), &profile)?;
      router.route_simple("profile")
    }),
    None => crate::redir(&session, &cfg, req, |_ctx, router| router.route_simple("profile"))
  }
}
