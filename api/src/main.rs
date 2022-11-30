pub mod account;
pub mod database;
pub mod routes;
pub mod util;

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
        .mount("/", routes![routes::index])
        .mount("/login", routes![routes::login::index])
        .mount("/signup", routes![routes::signup::index])
        .mount("/users", routes![routes::users::index])
        .mount(
            "/me",
            routes![
                routes::me::index,
                routes::me::friends::index,
                routes::me::friends::add,
                routes::me::friends::remove
            ],
        )
        .attach(cors.to_cors().unwrap())
}
