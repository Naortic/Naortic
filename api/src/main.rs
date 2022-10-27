pub mod util;
pub mod routes;

use routes::{github::github, index::index, user::username};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::all())
        .allowed_methods(
            vec![
                rocket::http::Method::Get,
                rocket::http::Method::Post,
                rocket::http::Method::Patch,
            ]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .mount("/", routes![index])
        .mount("/login", routes![github])
        .mount("/user", routes![username])
        .attach(cors.to_cors().unwrap())
}
