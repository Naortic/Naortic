pub mod database;
pub mod routes;
pub mod account;

use routes::{
    index::index,
    login::email::email,
    user::{
        friends::{add::friends_add, remove::friends_remove, friends},
        name::name,
        index as user_index,
    },
};

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
        .mount("/login", routes![email])
        .mount("/user", routes![user_index, friends, friends_add, friends_remove, name])
        .attach(cors.to_cors().unwrap())
}
