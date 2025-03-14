use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/auth");
    cfg.service(scope);
}
