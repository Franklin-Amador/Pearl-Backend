pub fn get_index() -> &'static str {
    "Hello, Actix!"
}

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_web::web::resource("/").route(actix_web::web::get().to(get_index)));
}