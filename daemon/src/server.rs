use std::thread;

use rocket::{http::Method, response::status, State};
use rocket_contrib::serve::StaticFiles;
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};

use crate::app::App;

pub fn create(app: App) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let api = routes![get_settings, set_setting];

        rocket::ignite()
            .attach(make_cors())
            .manage(app)
            .mount("/", StaticFiles::from("../ui/build/"))
            .mount("/api", api)
            .launch();
    })
}

#[get("/settings")]
fn get_settings(state: State<App>) -> Result<String, serde_json::Error> {
    state.lock().unwrap().get_settings().to_string()
}

#[post("/settings", data = "<settings>")]
fn set_setting(
    settings: String,
    state: State<App>,
) -> Result<status::Accepted<()>, status::BadRequest<()>> {
    match state.lock().unwrap().set_settings(settings) {
        Ok(_) => Ok(status::Accepted(None)),
        Err(_) => Err(status::BadRequest(None)),
    }
}

fn make_cors() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&["http://localhost:3000"]),
        allowed_methods: vec![Method::Options, Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        ..Default::default()
    }
    .to_cors()
    .unwrap()
}
