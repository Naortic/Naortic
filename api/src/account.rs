use serde::Serialize;

/// # Account
/// Data that is returned by the API in JSON format
/// # Nullable values
/// Any value which is of type `Option<T>` may not be sent due to security reasons,
/// Any value not worked as `Option<T>` will always be sent.
#[derive(Clone, Serialize, Debug)]
pub struct Account {
    pub name: String,
    pub password: Option<String>,
    pub token: Option<String>,
    pub email: Option<String>,
    pub friends: Option<Vec<String>>,
    pub snowflake: String,
    pub avatar: Option<String>,
}
