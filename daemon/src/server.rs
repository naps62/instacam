use std::thread;

use rocket::{http::Method, response::status, State};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error};

use crate::app::App;

pub fn create(app: App) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let routes = routes![get_settings, set_setting];

        rocket::ignite()
            .attach(make_cors())
            .manage(app)
            .mount("/", routes)
            .launch();
    })
}

fn make_cors() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::some_exact(&["http://localhost:3000"]),
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        ..Default::default()
    }
    .to_cors()
    .unwrap()
}

#[get("/settings")]
fn get_settings(state: State<App>) -> String {
    state.lock().unwrap().get_settings()
}

#[post("/settings", data = "<settings>")]
fn set_setting(settings: String, state: State<App>) -> status::Accepted<String> {
    state.lock().unwrap().set_settings(settings);

    status::Accepted(None)
}
