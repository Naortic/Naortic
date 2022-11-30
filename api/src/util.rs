use rocket::{Request, serde::json::Json, response, http::Status};
use serde::Serialize;
use snowflake::SnowflakeIdBucket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ErrorRes {
    error: String,
}

#[derive(Debug)]
pub struct Error(pub anyhow::Error);

impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(error: E) -> Self {
        Error(error.into())
    }
}

impl<'r, 'o: 'r> response::Responder<'r, 'o> for Error {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'o> {
        let mut res = Json(ErrorRes {
            error: format!("{:?}", self.0),
        })
        .respond_to(request)?;

        res.set_status(Status::InternalServerError);
        Ok(res)
    }
}

pub fn snowflake() -> String {
    let mut bucket = SnowflakeIdBucket::new(4, 2);

    bucket.get_id().to_string()
}
